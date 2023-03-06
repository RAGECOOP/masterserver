use actix_web::{
  web,
  Responder,
  HttpResponse,
  HttpRequest
};

pub(crate) async fn server(req: HttpRequest, mut info: web::Json<crate::servers::structs::Server>) -> impl Responder {
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
  if let Some(r) = req.headers().get("cf-ipcountry") {
    info.country = String::from(r.to_str().unwrap());
  }

  // Make sure the length of this data is not too long
  if let Some(x) = check_length("name", info.name.len()) {
    return HttpResponse::BadRequest().body(x);
  }

  // Make sure the length of this data is not too long
  if let Some(x) = check_length("description", info.description.len()) {
    return HttpResponse::BadRequest().body(x);
  }

  // Make sure the length of this data is not too long
  if let Some(x) = check_length("website", info.website.len()) {
    return HttpResponse::BadRequest().body(x);
  }

  // Make sure the length of this data is not too long
  if let Some(x) = check_length("country", info.country.len()) {
    return HttpResponse::BadRequest().body(x);
  }

  // Make sure the length of this data is not too long
  if let Some(x) = check_length("public_key_modulus", info.public_key_modulus.len()) {
    return HttpResponse::BadRequest().body(x);
  }

  // Make sure the length of this data is not too long
  if let Some(x) = check_length("public_key_exponent", info.public_key_exponent.len()) {
    return HttpResponse::BadRequest().body(x);
  }

  crate::servers::update_or_insert(&mut info);

  HttpResponse::Ok().body("OK")
}

fn check_length(name: &str, length: usize) -> Option<&str> {
  match name {
    "name" if length > 25 => Some("Your `name` is too long!"),
    "description" if length > 390 => Some("Your `description` is too long!"),
    "website" if length > 50 => Some("Your `website` is too long!"),
    "country" if length > 3 => Some("Your `country` is too long!"),
    "public_key_modulus" if length > 344 => Some("Your `public_key_modulus` is too long!"),
    "public_key_exponent" if length > 16 => Some("Your `public_key_exponent` is too long!"),
    _ => None
  }
}