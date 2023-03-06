#[macro_use]
extern crate serde_json;

use actix_web::{
  dev::Service as _,
  HttpServer,
  App,
  web
};

mod logger;
mod config;
mod servers;
mod routes;

static mut CURRENT_DIR: String = String::new();
pub(crate) fn get_current_dir() -> &'static String {
  unsafe {
    &CURRENT_DIR
  }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  unsafe {
    CURRENT_DIR = if let Ok(r) = std::env::current_dir() {
      format!("{}", r.display())
    } else { // Shouldn't happen
      String::from(".\\")
    };
  };

  let conf = config::load_config();

  logger::log("info", format!("port: {}", conf.server.port));
  logger::log("info", format!("workers: {}", conf.server.workers));

  logger::log("starting", format!("server on http://127.0.0.1:{}", conf.server.port));
  HttpServer::new(|| {
    let cors = actix_cors::Cors::default()
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
      .route("/", web::post().to(routes::post::server))
      .route("/", web::get().to(routes::get::server_list))
      .route("/all", web::get().to(routes::get::all))
      .route("/count", web::get().to(routes::get::count))
  })
  .workers(conf.server.workers as usize)
  .bind(("127.0.0.1", conf.server.port))?
  .run()
  .await
}
