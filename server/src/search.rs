// Search backend client.
//
// Lighthouse runs against either Elasticsearch or OpenSearch. The two speak
// the same REST API for the operations we use (search, bulk, delete_by_query).
// We use the `opensearch` crate (a fork of `elasticsearch-rs`) because it
// sends plain `application/json` headers, which both servers accept — whereas
// the v8 `elasticsearch` crate sends `compatible-with=8` headers that
// OpenSearch 2.x rejects with HTTP 406.
//
// `search_backend` is informational (logged at startup); the wire protocol is
// the same either way. Configuration env vars use the `search_*` prefix;
// legacy `elastic_*` names are still honored.

use once_cell::sync::Lazy;
use opensearch::{
    auth::Credentials,
    cert::{Certificate, CertificateValidation},
    http::transport::{SingleNodeConnectionPool, TransportBuilder},
    OpenSearch,
};
use url::Url;

use crate::globals::environment_variables;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchBackend {
    Elasticsearch,
    OpenSearch,
}

impl SearchBackend {
    fn from_env() -> Self {
        match env_var("search_backend").as_deref() {
            Some(s) if s.eq_ignore_ascii_case("opensearch") => SearchBackend::OpenSearch,
            Some(s) if s.eq_ignore_ascii_case("elasticsearch") => SearchBackend::Elasticsearch,
            Some(other) => panic!(
                "Unknown 'search_backend' value '{}'. Expected 'elasticsearch' or 'opensearch'.",
                other
            ),
            None => SearchBackend::Elasticsearch,
        }
    }
}

// Read a `search_*` env var, falling back to the legacy `elastic_*` name so
// existing deployments keep working without re-templating their secrets.
fn env_var(key: &str) -> Option<String> {
    if let Some(v) = environment_variables::get(key) {
        return Some(v);
    }
    if let Some(rest) = key.strip_prefix("search_") {
        return environment_variables::get(&format!("elastic_{}", rest));
    }
    None
}

fn require_env(key: &str) -> String {
    env_var(key).unwrap_or_else(|| {
        panic!(
            "Missing '{}' env variable (or its legacy 'elastic_*' equivalent).",
            key
        )
    })
}

pub static BACKEND: Lazy<SearchBackend> = Lazy::new(SearchBackend::from_env);

pub static CLIENT: Lazy<OpenSearch> = Lazy::new(|| {
    let backend = *BACKEND;
    log::info!("Search backend: {:?}", backend);

    let credentials = Credentials::Basic(
        require_env("search_username"),
        require_env("search_password"),
    );
    let url = Url::parse(&require_env("search_host")).expect("Failed to parse 'search_host' URL");
    let conn_pool = SingleNodeConnectionPool::new(url);

    // TLS validation policy:
    //   --no-index-tls            → skip validation entirely (dev / self-signed
    //                                without a CA on hand). Must be set
    //                                explicitly; we never silently disable.
    //   search_ca_cert_path=...   → pin to a private CA (in-cluster ES/OS
    //                                fronted by cert-manager).
    //   neither                   → validate against the system trust store.
    //                                Required for managed providers like
    //                                DigitalOcean whose endpoints use
    //                                publicly-trusted certs.
    let no_tls = std::env::args().any(|a| a == "--no-index-tls");
    let cert_validation = if no_tls {
        log::warn!("--no-index-tls set: skipping search backend TLS validation");
        CertificateValidation::None
    } else if let Some(path) = env_var("search_ca_cert_path") {
        let pem = std::fs::read(&path).expect("Failed to read 'search_ca_cert_path'.");
        let ca = Certificate::from_pem(&pem).expect("Failed to parse CA certificate.");
        CertificateValidation::Full(ca)
    } else {
        CertificateValidation::Default
    };

    let transport = TransportBuilder::new(conn_pool)
        .auth(credentials)
        .cert_validation(cert_validation)
        .build()
        .expect("Failed to build search transport.");

    OpenSearch::new(transport)
});
