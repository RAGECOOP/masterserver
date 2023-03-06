use actix_web::{
  Responder,
  HttpResponse,
  http::header::ContentType
};

pub(crate) async fn server_list() -> impl Responder {
  let servers = crate::servers::get_list();
  HttpResponse::Ok().content_type(ContentType::json()).body(serde_json::to_string(&servers).unwrap())
}

pub(crate) async fn count() -> impl Responder {
  let servers = crate::servers::get_list();
  HttpResponse::Ok().content_type(ContentType::json()).body(json!({
    "server_count": servers.len(),
    "player_count": _get_total_player_count(&servers)
  }).to_string())
}

pub(crate) async fn all() -> impl Responder {
  let servers = crate::servers::get_list();
  HttpResponse::Ok().content_type(ContentType::json()).body(json!({
    "server_count": servers.len(),
    "player_count": _get_total_player_count(&servers),
    "servers": servers
  }).to_string())
}

fn _get_total_player_count(servers: &Vec<crate::servers::structs::Server>) -> usize {
  let mut result: usize = 0;
  for i in servers.iter() {
    result += i.players as usize;
  }
  result
}