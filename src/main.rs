#[macro_use]
extern crate serde_json;

use actix_web::{
  dev::Service as _,
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
  let conf = config::load_config();

  logger::log("info", format!("port: {}", conf.server.port));
  logger::log("info", format!("workers: {}", conf.server.workers));

  logger::log("starting", format!("server on http://127.0.0.1:{}", conf.server.port));
  HttpServer::new(|| {
    let cors = Cors::default()
        .allow_any_origin()
        .allowed_methods(vec!["GET", "POST"]);
    
    App::new()
        .wrap(cors)
        .wrap_fn(|req, srv| {
          servers::cleanup();
          let fut = srv.call(req);
          async {
            let res = fut.await?;
            Ok(res)
          }
        })
        .route("/", web::post().to(post::server))
        .route("/", web::get().to(get::list))
        .route("/count", web::get().to(get::count))
  })
  .workers(conf.server.workers as usize)
  .bind(("127.0.0.1", conf.server.port))?
  .run()
  .await
}
