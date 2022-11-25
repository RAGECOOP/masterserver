use actix_web::{
  Responder,
  HttpResponse,
  http::header::ContentType
};

pub async fn list() -> impl Responder {
  HttpResponse::Ok().content_type(ContentType::json()).body(serde_json::to_string(&crate::servers::get_all()).unwrap())
}

pub async fn count() -> impl Responder {
  let (servers, players) = crate::servers::get_count();
  HttpResponse::Ok().content_type(ContentType::json()).body(json!({
    "servers": servers,
    "players": players
  }).to_string())
}