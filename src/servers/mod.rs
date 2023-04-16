use std::sync::{
  Mutex,
  MutexGuard
};
use std::time::SystemTime;

pub(crate) mod structs;

/// All servers are stored in this variable.
static mut SERVER_LIST: Mutex<Vec<structs::Server>> = Mutex::new(Vec::new());
/// Lock `SERVER_LIST` for other threads and call a function.
fn _server_list_callback(callback: &mut dyn FnMut(&mut MutexGuard<Vec<structs::Server>>)) {
  match unsafe { SERVER_LIST.lock() } {
    Ok(mut r) => callback(&mut r),
    Err(e) => crate::logger::log("error", format!("something went wrong while trying to lock `SERVER_LIST`\n{}", e))
  }
}

/// Get a clone of the current server list.
pub(crate) fn get_list() -> Vec<structs::Server> {
  let mut result: Vec<structs::Server> = Vec::new();
  _server_list_callback(&mut |list| { result = list.clone(); });
  result
}

/// Check if there is a server with the same IP address and port in the server list so we can update the server.
/// 
/// Otherwise we add this server to the server list.
pub(crate) fn update_or_insert(info: &mut structs::Server) {
  _server_list_callback(&mut |list| {
    // try to get the index of the current server position in our list by its address and port
    let index = list.iter().position(|r| r.address == info.address && r.port == info.port);

     // get the current timestamp as seconds in `u64`
    let current_timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

    // check if this server already exists.
    // of this server is not in our list, we will add it
    if index.is_none() {
      info.filter_bad_words();
      info.last_update = current_timestamp;
      
      list.push(info.clone());
    } else {
      // get a reference from the server via "index" and update some data
      let mut server = list.get_mut(index.unwrap()).unwrap();
      server.players = info.players;
      server.last_update = current_timestamp;
    }
  });
}

/// Remove all servers from our server list that have not been updated for more than 12 seconds.
pub(crate) fn cleanup() {
  _server_list_callback(&mut |list| {
    // get the current timestamp as seconds in `u64`
    let current_timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();

    // keep all elements that are true and remove others
    list.retain(|x| current_timestamp - x.last_update <= 12);
  });
}