#[macro_use]
extern crate serde_json;

use actix_web::{
  HttpServer,
  App,
  web
};

mod logger;
mod config;
mod servers;
mod get;
mod post;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  config::load_config("./config.toml");
  let conf = config::get_config();

  HttpServer::new(move || {
    App::new()
        .route("/", web::post().to(post::server))
        .route("/", web::get().to(get::server_list))
        .route("/count", web::get().to(get::server_count))
  })
  .workers(conf.server.workers as usize)
  .bind(("127.0.0.1", conf.server.port))?
  .run()
  .await
}
