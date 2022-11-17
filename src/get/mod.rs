use actix_web::{
  Responder,
  HttpResponse,
  http::header::ContentType
};

pub async fn server_list() -> impl Responder {
  HttpResponse::Ok().content_type(ContentType::json()).body(serde_json::to_string(crate::servers::get_servers()).unwrap())
}

pub async fn server_count() -> impl Responder {
  HttpResponse::Ok().content_type(ContentType::json()).body(json!({
    "id": 2,
    "email": 3
  }).to_string())
}