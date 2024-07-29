use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};

use crate::template::global_tera;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerError {
    InternalServerError(String),
    NotFound(String),
}

impl From<reqwest::Error> for ServerError {
    fn from(err: reqwest::Error) -> ServerError {
        log::error!("type=\"reqwest::Error\" error=\"{:?}\"", err);
        ServerError::InternalServerError(err.to_string())
    }
}

impl From<sqlx::Error> for ServerError {
    fn from(err: sqlx::Error) -> ServerError {
        log::error!("type=\"sqlx::Error\" error=\"{:?}\"", err);
        ServerError::InternalServerError(err.to_string())
    }
}

impl From<std::io::Error> for ServerError {
    fn from(err: std::io::Error) -> ServerError {
        log::error!("type=\"std::io::Error\" error=\"{:?}\"", err);
        ServerError::InternalServerError(err.to_string())
    }
}

impl From<tera::Error> for ServerError {
    fn from(err: tera::Error) -> ServerError {
        log::error!("type=\"tera::Error\" error=\"{:?}\"", err);
        ServerError::InternalServerError(err.to_string())
    }
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerError::InternalServerError(err) => {
                let mut ctx = tera::Context::new();
                ctx.insert("error", &err);
                let html = global_tera()
                    .render("internal_server_error.html", &ctx)
                    .unwrap();
                f.write_str(&html.as_str())
            }
            ServerError::NotFound(err) => {
                let mut ctx = tera::Context::new();
                ctx.insert("error", &err);
                let html = global_tera().render("not_found.html", &ctx).unwrap();
                f.write_str(&html.as_str())
            }
        }
    }
}

impl ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServerError::InternalServerError(_) => {
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .insert_header(("Content-Type", "text/html"))
                    .body(self.to_string())
            }
            ServerError::NotFound(_) => HttpResponse::build(StatusCode::NOT_FOUND)
                .insert_header(("Content-Type", "text/html"))
                .body(self.to_string()),
        }
    }
}
