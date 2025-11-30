use rocket::http::ContentType;
use rocket::get;

use crate::structs::*;

#[get("/opensearch.xml")]
pub async fn opensearch(headers: &Headers) -> (ContentType, String) {
    let mut _host_string: String = String::new();
    
    if headers.headers_map.get("origin").is_none() == false {
        _host_string = headers.headers_map.get("origin").unwrap().to_string();
    } else if headers.headers_map.get("host").is_none() == false {
        _host_string = headers.headers_map.get("host").unwrap().to_string();
    } else {
        return (ContentType::XML, "Missing headers.origin".to_string());
    }

    // IF missing http:// or https://, default to https://.
    if _host_string.starts_with("http://") == false && _host_string.starts_with("https://") == false {
        _host_string = format!("https://{}", _host_string);
    }

    let url = url::Url::parse(&_host_string).expect("Failed to parse url");
    let mut scheme = url.scheme().to_string();
    let mut host = url.host_str().unwrap().to_string();
    
    // Remove obvious escape characters in-case URL parser is bypassed.
    scheme = scheme.replace("\"", "").replace("/", "").replace("\\", "");
    host = host.replace("\"", "").replace("/", "").replace("\\", "");

    let output = format!("{}://{}:{}", scheme, host, url.port().unwrap_or(443));

    (ContentType::XML, r#"<?xml version="1.0" encoding="utf-8"?>
<OpenSearchDescription xmlns="http://a9.com/-/spec/opensearch/1.1/">
<ShortName>Lighthouse</ShortName>
<Description>Search Lighthouse</Description>
<InputEncoding>UTF-8</InputEncoding>
<LongName>Lighthouse</LongName>
<Url type="text/html" method="get" template="%DOMAIN%/query/?q={searchTerms}"/>
<Url type="application/x-suggestions+json" template="%DOMAIN%/query/?q={searchTerms}"/>
</OpenSearchDescription>"#.to_string().replace("%DOMAIN%", &output))
}