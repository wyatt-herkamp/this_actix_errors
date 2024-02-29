///! Exports of actix_web types
pub use actix_web::body::BoxBody;
use actix_web::http::header::HeaderValue;
pub use actix_web::http::header::{self, CONTENT_TYPE};
pub use actix_web::http::StatusCode;
pub use actix_web::web::Bytes;
pub use actix_web::HttpResponse;
pub use actix_web::ResponseError;

pub type ActixErrorResponse = HttpResponse<BoxBody>;
pub static CONTENT_TYPE_VALUE_STR: &str = "text/plain; charset=utf-8";
pub static CONTENT_TYPE_VALUE: HeaderValue = HeaderValue::from_static(CONTENT_TYPE_VALUE_STR);
#[cfg(feature = "tracing")]
pub use tracing::instrument;
