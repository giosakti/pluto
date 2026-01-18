use axum::Json;
use serde_json::json;

use crate::response::{self, ProblemDetails};

pub async fn example_bad_request() -> Result<Json<serde_json::Value>, ProblemDetails> {
    Err(response::bad_request(
        json!({ "message": "example bad request" }).to_string(),
    ))
}

pub async fn example_not_found() -> Result<Json<serde_json::Value>, ProblemDetails> {
    Err(response::not_found("example not found").with_instance("/example-not-found"))
}

pub async fn example_internal_error() -> Result<Json<serde_json::Value>, ProblemDetails> {
    Err(response::internal_error("example internal error").with_instance("/example-internal-error"))
}
