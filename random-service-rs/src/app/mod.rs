use actix_web::{dev, web, guard, App, HttpResponse, HttpServer, Result};
use crate::error::{R, Resp, ErrorResponse};
use crate::state::{SvrState, AuthState};


mod auth;

async fn index() -> Resp {
    R::ok("Random Service Rust API server.").to_json_result()
}

pub async fn render_404() -> Resp {
    Err(ErrorResponse::NotFound)
}

pub fn root_config(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/", web::get().to(index))
        .service(
            web::resource("/token")
                .guard(guard::Header("content-type", "application/json"))
                .route(web::get()   .to(auth::get_token))
                .route(web::put()   .to(auth::put_token))
                .route(web::delete().to(auth::del_token))
        )
        .service(
            web::resource("/token/test")
                .guard(guard::Header("content-type", "application/json"))
                .route(web::get().to(auth::get_token_test))
        )
    ;
}