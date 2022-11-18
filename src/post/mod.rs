use actix_web::{
  web,
  Responder,
  HttpResponse
};

pub async fn server(mut info: web::Json<crate::servers::Server>) -> impl Responder {
  if !_check_info(&info) {
    return HttpResponse::BadRequest().body("BAD REQUEST");
  }

  crate::servers::update_or_insert(&mut info);

  HttpResponse::Ok().body("OK")
}

fn _check_info(info: &crate::servers::Server) -> bool {
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