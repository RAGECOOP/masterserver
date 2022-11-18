#[macro_use]
extern crate serde_json;

use actix_web::{
  HttpServer,
  App,
  web
};
use actix_cors::Cors;

mod logger;
mod config;
mod servers;
mod get;
mod post;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  config::load_config("./config.toml");
  let conf = config::get_config();

  logger::log("info", format!("port: {}", conf.server.port).as_str());
  logger::log("info", format!("workers: {}", conf.server.workers).as_str());

  logger::log("starting", format!("server on http://127.0.0.1:{}", conf.server.port).as_str());
  HttpServer::new(|| {
    let cors = Cors::default()
        .allow_any_origin()
        .allowed_methods(vec!["GET", "POST"]);
    
    App::new()
        .wrap(cors)
        .route("/", web::post().to(post::server))
        .route("/", web::get().to(get::server_list))
        .route("/count", web::get().to(get::server_count))
  })
  .workers(conf.server.workers as usize)
  .bind(("127.0.0.1", conf.server.port))?
  .run()
  .await
}
