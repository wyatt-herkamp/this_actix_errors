use actix_web::{get, web, App, HttpServer, Responder};
use actix_web::http::header;
use actix_web::web::Bytes;
use this_actix_error::ActixError;
use thiserror::Error;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

#[derive(Error, Debug, ActixError)]
pub enum WebError {
    #[status_code(500)]
    #[message("Failed to read ")]
    #[error("Failed to read {0}")]
    Io(#[from] std::io::Error),
    #[status_code(500)]
    #[error("Test Error")]
    Test
}