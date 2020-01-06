use actix_web::{dev, web, App, HttpResponse, HttpServer, Result};
use crate::error::{R, Resp, ErrorResponse};
use crate::state::{SvrState, AuthState};

async fn index() -> Resp {
    R::ok("Random Service Rust API server.").to_json_result()
}

pub async fn render_404() -> Resp {
    Err(ErrorResponse::NotFound)
}

pub fn root_config(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/", web::get().to(index))
    ;
}