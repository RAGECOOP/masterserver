use serde::{
  Serialize,
  Deserialize
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Server {
  pub address: Option<String>,
  pub port: Option<String>,
  pub name: Option<String>,
  pub version: Option<String>,
  pub players: Option<String>,
  #[serde(rename = "maxPlayers")]
  pub max_players: Option<String>,
  pub country: Option<String>,
  pub description: Option<String>,
  pub website: Option<String>,
  #[serde(rename = "gameMode")]
  pub game_mode: Option<String>,
  pub language: Option<String>,
  #[serde(rename = "useP2P")]
  pub use_p2p: Option<bool>,
  #[serde(rename = "useZT")]
  pub use_zt: Option<bool>,
  #[serde(rename = "ztID")]
  pub zt_id: Option<String>,
  #[serde(rename = "ztAddress")]
  pub zt_address: Option<String>,
  #[serde(rename = "publicKeyModules")]
  pub public_key_modules: Option<String>,
  #[serde(rename = "publicKeyExponent")]
  pub public_key_exponent: Option<String>
}

static mut SERVER_LIST: Vec<Server> = Vec::new();
pub fn get_servers() -> &'static Vec<Server> {
  unsafe {
    SERVER_LIST.as_ref()
  }
}

pub fn insert_server(info: &Server) {
  unsafe {
    SERVER_LIST.push(info.clone());
  }
}

pub fn update_server(info: &Server) -> bool {
  unsafe {
    let index = SERVER_LIST.iter().position(|r| r.address.as_ref().unwrap() == info.address.as_ref().unwrap() && r.port.as_ref().unwrap() == info.port.as_ref().unwrap());
    if index.is_none() {
      return false;
    }

    let mut server = SERVER_LIST.get_mut(index.unwrap()).unwrap();
    server.players = info.players.clone();
  }

  true
}

pub fn server_exists(info: &Server) -> bool {
  unsafe {
    SERVER_LIST.iter().position(|r| r.address.as_ref().unwrap() == info.address.as_ref().unwrap() && r.port.as_ref().unwrap() == info.port.as_ref().unwrap()).is_some()
  }
}