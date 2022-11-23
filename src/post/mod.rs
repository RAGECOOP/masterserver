use actix_web::{
  web,
  Responder,
  HttpResponse,
  HttpRequest
};

pub async fn server(req: HttpRequest, mut info: web::Json<crate::servers::structs::Server>) -> impl Responder {
  // Get the real IP address with Cloudflare
  match req.headers().get("cf-connecting-ip") {
    Some(r) => {
      let addr = r.to_str().unwrap();

      // Check if the IP address a valid IPv4 so we override the info.address because
      // maybe this guy changed his address with the source code
      if addr.parse::<std::net::Ipv4Addr>().is_ok() {
        info.address = addr.to_string();
      }
    },
    None => {
      return HttpResponse::InternalServerError().body("Missing header!");
    }
  };

  // Try using Cloudflare to get the original country from the IP address
  match req.headers().get("cf-ipcountry") {
    Some(r) => info.country = String::from(r.to_str().unwrap()),
    None => {}
  }

  // We don't host books!
  if info.name.len() > 25
  || info.description.len() > 390
  || info.website.len() > 50
  || info.country.len() > 3
  || info.public_key_modulus.len() > 344
  || info.public_key_exponent.len() > 16 {
    return HttpResponse::BadRequest().body("Your server name, description, website, country, public-key-modulus or public-key-exponent length is too long!");
  }

  crate::servers::update_or_insert(&mut info);

  HttpResponse::Ok().body("OK")
}