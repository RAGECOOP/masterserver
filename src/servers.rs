use std::sync::Mutex;
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

static mut SERVER_LIST: Mutex<Vec<Server>> = Mutex::new(Vec::new());
pub fn get_all() -> Vec<Server> {
  unsafe {
    // Lock
    let list = SERVER_LIST.lock().unwrap();

    // Clone the list of servers
    let result = list.clone();

    // Unlock
    std::mem::drop(list);
    
    // Return
    result
  }
}
pub fn get_count() -> (usize, usize) {
  unsafe {
    // Lock
    let list = SERVER_LIST.lock().unwrap();

    let total_servers = list.len();
    let mut total_players = 0;
    for i in list.iter() {
      total_players += i.players.as_ref().unwrap().parse::<usize>().unwrap();
    }

    // Unlock
    std::mem::drop(list);

    // Return
    (total_servers, total_players)
  }
}

/// Update or add a server
pub fn update_or_insert(info: &Server) {
  unsafe {
    // Lock
    let mut list = SERVER_LIST.lock().unwrap();

    // Try to get the index of the current server position in our list by its address and port
    let index = list.iter().position(|r| r.address.as_ref().unwrap() == info.address.as_ref().unwrap() && r.port.as_ref().unwrap() == info.port.as_ref().unwrap());
    // Check if this server already exists.
    // If this server is not in our list, we will add it
    if index.is_none() {
      *&list.push(info.clone());
    } else {
      // Get the server via `index` and change some values
      let mut server = list.get_mut(index.unwrap()).unwrap();
      server.players = info.players.clone();
      server.max_players = info.max_players.clone();
    }

    // Unlock
    std::mem::drop(list);
  }
}