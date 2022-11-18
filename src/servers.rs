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
  pub address: String,
  pub port: u16,
  pub name: String,
  pub version: String,
  pub players: u16,
  #[serde(rename = "maxPlayers")]
  pub max_players: u16,
  pub country: Option<String>,
  pub description: Option<String>,
  pub website: Option<String>,
  #[serde(rename = "gameMode")]
  pub game_mode: Option<String>,
  pub language: Option<String>,
  #[serde(rename = "useP2P")]
  pub use_p2p: bool,
  #[serde(rename = "useZT")]
  pub use_zt: bool,
  #[serde(rename = "ztID")]
  pub zt_id: Option<String>,
  #[serde(rename = "ztAddress")]
  pub zt_address: Option<String>,
  #[serde(rename = "publicKeyModulus")]
  pub public_key_modulus: String,
  #[serde(rename = "publicKeyExponent")]
  pub public_key_exponent: String,
  // We add these following fields ourselves
  #[serde(rename = "playerStats", skip_deserializing)]
  pub player_stats: PlayerStats,
  #[serde(skip)]
  pub last_update: u64
}

impl Server {
  // This function checks the name and description for bad words
  /// TODO!
  #[allow(dead_code)]
  pub fn contains_bad_words(&self) -> bool {
    false
  }
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct PlayerStats {
  pub players: Vec<u16>,
  #[serde(skip)]
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
    total_players += i.players as usize;
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
  let index = list.iter().position(|r| r.address == info.address && r.port == info.port);
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
    info.player_stats = PlayerStats {
      players: vec![0, 0, 0, 0, 0, 0],
      last_update: current_timestamp
    };
    info.last_update = current_timestamp;
    //println!("{}", serde_json::to_string(&info).unwrap());
    *&list.push(info.to_owned());
  } else {
    // Get the server via `index` and change some values
    let mut server = list.get_mut(index.unwrap()).unwrap();
    server.players = info.players;

    // Check if last update older than 10 minutes.
    // If not and the new player count is higher than the old highest peak, replace that value
    if current_timestamp - server.player_stats.last_update > 600 {
      server.player_stats.players.remove(0);
      server.player_stats.players.push(server.players);

      server.player_stats.last_update = current_timestamp;
    } else if server.players > *server.player_stats.players.last().unwrap() {
      server.player_stats.players[5] = server.players;
    }
    
    server.last_update = current_timestamp;
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
    // Check if the update is older than 12 seconds
    if current_timestamp - i.last_update > 12 {
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