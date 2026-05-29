// Reverse proxy for the services that used to sit behind nginx.
//
// axum now binds the container's port 80 and serves the API directly. The
// other two processes started by start-webservers.sh — the Next.js frontend
// (:3000) and the Guard auth server (:8000) — are reached by forwarding the
// requests that aren't handled by an API route:
//
//   /guard/*  -> Guard server   (config: guard_proxy_target)
//   anything  -> Next.js frontend (config: frontend_proxy_target, the fallback)

use std::env;

use axum::body::Body;
use axum::extract::Request;
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};

use once_cell::sync::Lazy;
use url::Url;

static HTTP_CLIENT: Lazy<reqwest::Client> = Lazy::new(reqwest::Client::new);

fn frontend_target() -> String {
    env::var("frontend_proxy_target").unwrap_or_else(|_| "http://localhost:3000".to_string())
}

fn guard_target() -> String {
    env::var("guard_proxy_target").unwrap_or_else(|_| "http://localhost:8000".to_string())
}

/// Fallback handler: forwards everything that didn't match an API route to the
/// Next.js frontend.
pub async fn frontend_proxy(req: Request) -> Response {
    proxy(&frontend_target(), req).await
}

/// Forwards `/guard/*` to the Guard auth server.
pub async fn guard_proxy(req: Request) -> Response {
    proxy(&guard_target(), req).await
}

async fn proxy(target_base: &str, req: Request) -> Response {
    let (parts, body) = req.into_parts();

    // Build the upstream URL via the URL parser rather than string
    // concatenation: the scheme/host/port come solely from `target_base`, and
    // `set_path`/`set_query` only touch the path and query components. A
    // crafted request path therefore can't escape to a different host.
    let mut url = match Url::parse(target_base) {
        Ok(url) => url,
        Err(e) => {
            log::error!("invalid proxy target {}: {}", target_base, e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Invalid proxy target").into_response();
        }
    };
    url.set_path(parts.uri.path());
    url.set_query(parts.uri.query());

    let body_bytes = match axum::body::to_bytes(body, usize::MAX).await {
        Ok(bytes) => bytes,
        Err(_) => return (StatusCode::BAD_REQUEST, "Failed to read request body").into_response(),
    };

    let mut request_builder = HTTP_CLIENT.request(parts.method, url.clone()).body(body_bytes);
    for (name, value) in parts.headers.iter() {
        // The upstream host differs; let reqwest set its own Host header.
        if name == header::HOST {
            continue;
        }
        request_builder = request_builder.header(name, value);
    }

    let upstream = match request_builder.send().await {
        Ok(response) => response,
        Err(e) => {
            log::error!("proxy to {} failed: {}", url, e);
            return (StatusCode::BAD_GATEWAY, "Upstream service unavailable").into_response();
        }
    };

    let status = upstream.status();
    let upstream_headers = upstream.headers().clone();
    let mut response = Response::new(Body::from_stream(upstream.bytes_stream()));
    *response.status_mut() = status;

    let headers = response.headers_mut();
    for (name, value) in upstream_headers.iter() {
        headers.insert(name, value.clone());
    }

    response
}
