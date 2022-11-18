use actix_web::{
  web,
  Responder,
  HttpResponse,
  HttpRequest
};

pub async fn server(req: HttpRequest, mut info: web::Json<crate::servers::Server>) -> impl Responder {
  if !_check_info(&info) {
    return HttpResponse::BadRequest().body("BAD REQUEST!");
  }

  // Get the real IP address with Cloudflare
  match req.headers().get("cf-connecting-ip") {
    Some(r) => {
      let addr = r.to_str().unwrap();

      // Check if the IP address a valid IPv4 so we override the info.address because
      // maybe this guy changed his address with the source code
      if addr.parse::<std::net::Ipv4Addr>().is_ok() {
        info.address = Some(addr.to_string());
      }
    },
    None => {
      return HttpResponse::InternalServerError().body("Missing header!");
    }
  };

  // Try using Cloudflare to get the original country from the IP address
  match req.headers().get("cf-ipcountry") {
    Some(r) => info.country = Some(String::from(r.to_str().unwrap())),
    None => {}
  }

  if !crate::servers::update_or_insert(&mut info) {
    return HttpResponse::InternalServerError().body("Something went wrong!");
  }

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