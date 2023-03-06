use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Data {
  pub server: Server
}

#[derive(Deserialize)]
pub(crate) struct Server {
  pub port: u16,
  pub workers: u16
}