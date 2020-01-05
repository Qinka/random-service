use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use serde_yaml::{from_reader, Value};
use argparse::{ArgumentParser, Store};
use env_logger;


mod error;
mod db;
mod state;

use error::{R};
use state::{SvrState, AuthState};

async fn index() -> error::Resp {
    error::R::ok("Random Service Rust API server.\n").to_json_result()
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    let mut cfg_path = "config.yaml".to_string();
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut cfg_path).add_option(&["--config"], Store, "Config file");
        ap.parse_args_or_exit();
    }
    let file = std::fs::File::open(cfg_path)?;
    let config: Value = from_reader(file).unwrap();
    let mongo = match config.get("auth-mg") {
        None => panic!("Need ``auth-mg'' for author mongodb"),
        Some(config) =>
            db::create_mongo_client(
                config.get("uri").expect("Need ``auth-mg.uri'' the mongodb link uri").as_str().expect("``auth-mg.url'' should be string"),
                config.get("size").map(|size| size.as_u64().expect("``auth-mg.size'' should be int") as u32)
            )
    };

    match config.get("random-service-rs") {
        None => panic!("invailed config file, need field of random-service-py"),
        Some(config) => {
            // std::env::var("RUST_LOG")
            // std::env::set_var("RUST_LOG", "actix_web=info");
            env_logger::init();


            HttpServer::new(move || {
                App::new()
                    .data(SvrState{auth: AuthState{mongo: mongo.clone()}})
                    .wrap(Logger::default())
                    .wrap(Logger::new("%a %{User-Agent}i"))
                    .route("/", web::get().to(index))
            })
            .bind(format!("0.0.0.0:{}", config.get("port").expect("Need port (int)").as_u64().expect("port should be int")))?
            .run()
            .await
        }
    }
}
