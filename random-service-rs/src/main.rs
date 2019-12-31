use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use serde_yaml::{from_reader, Value};
use argparse::{ArgumentParser, Store};
use env_logger;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Random Service Rust API server.\n")
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

    match config.get("random-service-rs") {
        None => panic!("invailed config file, need field of random-service-py"),
        Some(config) => {
            std::env::set_var("RUST_LOG", "actix_web=info");
            env_logger::init();

            HttpServer::new(|| {
                App::new()
                    .wrap(Logger::default())
                    .wrap(Logger::new("%a %{User-Agent}i"))
                    .route("/", web::get().to(index))
            })
            .bind(format!("0.0.0.0:{}", config.get("port").unwrap().as_u64().unwrap()))?
            .run()
            .await
        }
    }
}
