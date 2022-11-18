use std::sync::{
  Mutex,
  MutexGuard
};
use std::time::SystemTime;

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
  #[serde(rename = "publicKeyModulus")]
  pub public_key_modulus: Option<String>,
  #[serde(rename = "publicKeyExponent")]
  pub public_key_exponent: Option<String>,
  #[serde(rename = "playerStats")]
  pub player_stats: Option<PlayerStats>,
  #[serde(rename = "lastUpdate")]
  pub last_update: Option<u64>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PlayerStats {
  pub players: Vec<i8>,
  #[serde(rename = "lastUpdate")]
  pub last_update: u64
}

static mut SERVER_LIST: Mutex<Vec<Server>> = Mutex::new(Vec::new());
pub fn get_all() -> Vec<Server> {
  let list: MutexGuard<Vec<Server>>;
  unsafe {
    // Lock
    list = SERVER_LIST.lock().unwrap();
  }

  // Clone the list of servers
  let result = list.clone();

  // Unlock
  std::mem::drop(list);
  
  // Return
  result
}
pub fn get_count() -> (usize, usize) {
  let list: MutexGuard<Vec<Server>>;
  unsafe {
    // Lock
    list = SERVER_LIST.lock().unwrap();
  }

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

/// Update or add a server
pub fn update_or_insert(info: &mut Server) -> bool {
  let mut list: MutexGuard<Vec<Server>>;
  unsafe {
    // Lock
    list = SERVER_LIST.lock().unwrap();
  }

  // Try to get the index of the current server position in our list by its address and port
  let index = list.iter().position(|r| r.address.as_ref().unwrap() == info.address.as_ref().unwrap() && r.port.as_ref().unwrap() == info.port.as_ref().unwrap());
  let current_timestamp = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
    Ok(r) => r.as_secs(),
    Err(e) => {
      crate::logger::log("error", &e.to_string());
      return false;
    }
  };
  // Check if this server already exists.
  // If this server is not in our list, we will add it
  if index.is_none() {
    info.player_stats = Some(PlayerStats {
      players: vec![0, 0, 0, 0, 0],
      last_update: current_timestamp
    });
    info.last_update = Some(current_timestamp);
    //println!("{}", serde_json::to_string(&info).unwrap());
    *&list.push(info.to_owned());
  } else {
    // Get the server via `index` and change some values
    let mut server = list.get_mut(index.unwrap()).unwrap();
    server.players = info.players.to_owned();
    server.max_players = info.max_players.to_owned();
    server.last_update = Some(current_timestamp);
  }

  // Unlock
  std::mem::drop(list);

  true
}

/// Clean the list of servers
pub fn cleanup() -> bool {
  let mut list: MutexGuard<Vec<Server>>;
  unsafe {
    // Lock
    list = SERVER_LIST.lock().unwrap();
  }

  // The new list of servers that will replace the old one
  let mut new_list: Vec<Server> = Vec::new();

  let current_timestamp = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
    Ok(r) => r.as_secs(),
    Err(e) => {
      crate::logger::log("error", &e.to_string());
      return false;
    }
  };
  for i in list.iter() {
    // Check if the update is older than 10 seconds
    if current_timestamp - i.last_update.unwrap() > 10 {
      continue;
    }

    new_list.push(i.clone());
  }

  // Replace the current server list with "new_list"
  *list = new_list;

  // Unlock
  std::mem::drop(list);

  true
}