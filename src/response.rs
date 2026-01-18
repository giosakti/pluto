use axum::Json;
use axum::http::HeaderValue;
use axum::http::StatusCode;
use axum::http::header;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

/// URN-style identifiers for RFC 7807 `type`.
pub const TYPE_BAD_REQUEST: &str = "urn:agnx:problem:bad-request";
pub const TYPE_INTERNAL_ERROR: &str = "urn:agnx:problem:internal-error";
pub const TYPE_NOT_FOUND: &str = "urn:agnx:problem:not-found";

/// RFC 7807 Problem Details response
#[derive(Debug, Serialize)]
pub struct ProblemDetails {
    pub r#type: String,
    pub title: String,
    pub status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
}

impl ProblemDetails {
    pub fn new(status: StatusCode, title: impl Into<String>) -> Self {
        Self {
            r#type: "about:blank".to_string(),
            title: title.into(),
            status: status.as_u16(),
            detail: None,
            instance: None,
        }
    }

    pub fn with_type(mut self, type_uri: impl Into<String>) -> Self {
        self.r#type = type_uri.into();
        self
    }

    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    pub fn with_instance(mut self, instance: impl Into<String>) -> Self {
        self.instance = Some(instance.into());
        self
    }
}

impl IntoResponse for ProblemDetails {
    fn into_response(self) -> Response {
        let mut pd = self;
        let status = StatusCode::from_u16(pd.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        pd.status = status.as_u16();

        (
            status,
            [(
                header::CONTENT_TYPE,
                HeaderValue::from_static("application/problem+json"),
            )],
            Json(pd),
        )
            .into_response()
    }
}

/// Common error responses
pub fn bad_request(detail: impl Into<String>) -> ProblemDetails {
    ProblemDetails::new(StatusCode::BAD_REQUEST, "Bad Request")
        .with_type(TYPE_BAD_REQUEST)
        .with_detail(detail)
}

pub fn internal_error(detail: impl Into<String>) -> ProblemDetails {
    ProblemDetails::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
        .with_type(TYPE_INTERNAL_ERROR)
        .with_detail(detail)
}

pub fn not_found(detail: impl Into<String>) -> ProblemDetails {
    ProblemDetails::new(StatusCode::NOT_FOUND, "Not Found")
        .with_type(TYPE_NOT_FOUND)
        .with_detail(detail)
}
