// Web layer: axum router, request extractors, and CORS middleware.

use std::collections::HashMap;
use std::convert::Infallible;

use axum::{
    body::Body,
    extract::FromRequestParts,
    http::{header, request::Parts, HeaderValue, Method, Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};

use crate::responses::*;
use crate::structs::*;

/// Builds the full application router.
pub fn router() -> Router {
    Router::new()
        .route("/opensearch.xml", get(crate::endpoint::misc::opensearch))
        .route(
            "/api/native-v1/query/list",
            get(crate::endpoint::query::query_list),
        )
        .route(
            "/api/native-v1/crawler/index",
            post(crate::endpoint::crawler::crawler_index),
        )
        .route(
            "/api/native-v1/crawler/queue",
            get(crate::endpoint::crawler::crawler_queue),
        )
        .route(
            "/api/native-v1/account/me",
            get(crate::endpoint::account::account_me),
        )
        .route(
            "/api/native-v1/account/list",
            get(crate::endpoint::account::account_list),
        )
        .route(
            "/api/native-v1/admin/index/job/list",
            get(crate::endpoint::admin::index::admin_index_list),
        )
        .route(
            "/api/native-v1/admin/index/job/update",
            post(crate::endpoint::admin::index::admin_index_update),
        )
}

/// Mirrors the previous Rocket CORS fairing: permissive headers on every
/// response, and a short-circuit 200 for CORS preflight (OPTIONS) requests.
pub async fn cors_middleware(request: Request<Body>, next: Next) -> Response {
    let is_preflight = request.method() == Method::OPTIONS;

    let mut response = if is_preflight {
        StatusCode::OK.into_response()
    } else {
        next.run(request).await
    };

    let headers = response.headers_mut();
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_ORIGIN,
        HeaderValue::from_static("*"),
    );
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_METHODS,
        HeaderValue::from_static("POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET"),
    );
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_HEADERS,
        HeaderValue::from_static("*"),
    );
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
        HeaderValue::from_static("true"),
    );
    headers.remove(header::SERVER);

    response
}

/// Replaces Rocket's `#[catch(500)]`: turns a panicking handler into a JSON
/// 500 instead of dropping the connection.
pub fn handle_panic(_err: Box<dyn std::any::Any + Send + 'static>) -> Response {
    let body = Json(error_message("server_error", "Internal server error"));
    (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
}

// Request extractor: the raw query string, used by `request_authentication`.
impl<S> FromRequestParts<S> for Query_string
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let query_params = parts
            .uri
            .query()
            .map(|query| query.to_owned())
            .unwrap_or_default();

        Ok(Query_string(query_params))
    }
}

// Request extractor: all request headers as a lowercase-keyed map.
impl<S> FromRequestParts<S> for Headers
where
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let headers_map = parts
            .headers
            .iter()
            .map(|(name, value)| {
                (
                    name.as_str().to_string(),
                    value.to_str().unwrap_or_default().to_string(),
                )
            })
            .collect::<HashMap<String, String>>();

        Ok(Headers { headers_map })
    }
}
