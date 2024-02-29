use actix_web::App;
use actix_web::HttpServer;
use this_actix_error::ActixError;
use thiserror::Error;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::util::SubscriberInitExt;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .finish()
        .init();

    HttpServer::new(|| {
        App::new()
            .wrap(TracingLogger::default())
            .service(io_error)
            .service(test_error)
            .service(two_error)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
#[actix_web::get("/io_error")]
async fn io_error() -> Result<String, WebError> {
    Err(WebError::Io(std::io::Error::new(
        std::io::ErrorKind::Other,
        "test",
    )))
}
#[actix_web::get("/test_error")]
async fn test_error() -> Result<String, WebError> {
    Err(WebError::Test)
}
#[actix_web::get("/two_error")]
async fn two_error() -> Result<String, WebError> {
    Err(WebError::Two { a: 1, b: 2 })
}

#[derive(Error, Debug, ActixError)]
pub enum WebError {
    #[status_code(500)]
    #[message("Failed to read ")]
    #[error("Failed to read {0}")]
    Io(#[from] std::io::Error),
    #[status_code(404)]
    #[error("Not Found")]
    Test,
    #[status_code(BAD_REQUEST)]
    #[error("Test Error {a} {b}")]
    Two { a: i32, b: i32 },
}
