use std::sync::{
  Mutex,
  MutexGuard
};
use std::time::SystemTime;

pub mod structs;

/// All servers are stored in this variable
static mut SERVER_LIST: Mutex<Vec<structs::Server>> = Mutex::new(Vec::new());

/// Get a copy of the current server list
pub fn get_list() -> Vec<structs::Server> {
  let mut result: Vec<structs::Server> = Vec::new();

  _server_list_callback(&mut |list| {
    result = list.clone();
  });

  // Return
  result
}

/// Update or add a server
pub fn update_or_insert(info: &mut structs::Server) {
  _server_list_callback(&mut |list| {
    // Try to get the index of the current server position in our list by its address and port
    let index = list.iter().position(|r| r.address == info.address && r.port == info.port);

     // Get the current timestamp as seconds in `u64`
    let current_timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

    // Check if this server already exists.
    // If this server is not in our list, we will add it
    if index.is_none() {
      info.filter_bad_words();

      info.player_stats = structs::PlayerStats {
        players: vec![0; 6],
        last_update: current_timestamp
      };
      info.last_update = current_timestamp;
      
      // Add the new server to `SERVER_LIST`
      list.push(info.to_owned());
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
  });
}

/// Clean the list of servers
pub fn cleanup() {
  _server_list_callback(&mut |list| {
    // Get the current timestamp as seconds in `u64`
    let current_timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

    // Keep all elements that are true and remove others
    list.retain(|x| current_timestamp - x.last_update <= 12);
  });
}

/// Lock `SERVER_LIST` for other threads, call a function and unlock `SERVER_LIST`
fn _server_list_callback(callback: &mut dyn FnMut(&mut MutexGuard<Vec<structs::Server>>)) {
  // Lock `SERVER_LIST` for other threads
  let mut list = unsafe {
    match SERVER_LIST.lock() {
      Ok(r) => r,
      Err(e) => {
        crate::logger::log("error", format!("something went wrong while trying to lock `SERVER_LIST`: {}", e.to_string()));
        return;
      }
    }
  };
  
  callback(&mut list);

  // Unlock `SERVER_LIST` for other threads
  std::mem::drop(list);
}