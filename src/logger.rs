/**
 * THIS IS JUST A SIMPLE CONSOLE LOGGER!
 */

use std::sync::{
  Mutex,
  MutexGuard
};

static mut _LOCK: Mutex<i8> = Mutex::new(0);

pub fn log(state: &str, text: &str) {
  let lock: MutexGuard<i8>;
  unsafe {
    lock = _LOCK.lock().unwrap();
  }

  use colored::*;

  let s = match state {
    "warning" => "WARNING".bold().bright_yellow(),
    "error" => "ERROR".bold().bright_red(),
    "creating" | "loading" | "successfully" | "starting" => state.to_uppercase().bold().bright_green(),
    _ => "INFO".bold()
  };

  println!("{} {}", s, text);

  // Unlock
  std::mem::drop(lock);
}