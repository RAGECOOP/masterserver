use actix_web::{
  web,
  Responder,
  HttpResponse
};

pub async fn server(info: web::Json<crate::servers::Server>) -> impl Responder {
  println!("{}", serde_json::to_string(&info).unwrap());
  if !validate_info(&info) {
    return HttpResponse::BadRequest().body("BAD REQUEST");
  }

  if !crate::servers::server_exists(&info) {
    crate::servers::insert_server(&info);
  } else if !crate::servers::update_server(&info) {
    return HttpResponse::BadRequest().body("Server could not be updated!");
  }

  HttpResponse::Ok().body("OK")
}

fn validate_info(info: &crate::servers::Server) -> bool {
  info.address.is_some()
  && info.port.is_some()
  && info.name.is_some()
  && info.version.is_some()
  && info.players.is_some()
  && info.max_players.is_some()
  && info.country.is_some()
  && info.description.is_some()
  && info.website.is_some()
  && info.game_mode.is_some()
  && info.language.is_some()
  && info.use_p2p.is_some()
  && info.use_zt.is_some()
  && info.zt_id.is_some()
  && info.zt_address.is_some()
  //&& info.public_key_modules.is_some()
  //&& info.public_key_exponent.is_some()
}