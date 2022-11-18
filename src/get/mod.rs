use actix_web::{
  Responder,
  HttpResponse,
  http::header::ContentType
};

pub async fn server_list() -> impl Responder {
  crate::servers::cleanup();
  HttpResponse::Ok().content_type(ContentType::json()).body(serde_json::to_string(&crate::servers::get_all()).unwrap())
}

pub async fn server_count() -> impl Responder {
  crate::servers::cleanup();
  let (servers, players) = crate::servers::get_count();
  HttpResponse::Ok().content_type(ContentType::json()).body(json!({
    "servers": servers,
    "players": players
  }).to_string())
}