use std::fmt::Display;

use actix_web::HttpResponse;
use anyhow::Result;
use http::HeaderValue;

pub type ApiResponse = Result<HttpResponse, ApiError>;

#[derive(Debug)]
pub struct ApiError(anyhow::Error);

impl<E> From<E> for ApiError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl actix_web::error::ResponseError for ApiError {
    fn status_code(&self) -> hyper::StatusCode {
        hyper::StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        tracing::error!("{:?}", self);
        let mut res = HttpResponse::new(self.status_code());
        // res.headers().
        res.headers_mut().insert(
            actix_web::http::header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        let error_raw = format!(r#"{{"error":"{}"}}"#, self);
        res.set_body(actix_web::body::BoxBody::new(error_raw))
    }
}
