//! THIS IS JUST A SIMPLE CONSOLE LOGGER!

use std::{
  env,
  sync::{
    Mutex,
    MutexGuard
  },
  path::Path,
  fs,
  fs::File,
  io::prelude::*
};
use chrono::Utc;
use colored::*;

static mut LOG_FILE: Mutex<Option<File>> = Mutex::new(None);
fn _log_file_callback(callback: &dyn Fn(&mut MutexGuard<Option<File>>)) {
  if let Ok(mut r) = unsafe { LOG_FILE.lock() } {
    callback(&mut r)
  }
}

fn create_log_file() {
  _log_file_callback(&|file| {
    // we already have a file
    if file.is_some() { return; }

    let working_dir = env::current_dir().expect("can't access current dir");
    fs::create_dir_all(format!("{}\\logs", working_dir.display())).expect("couldn't create `logs` dir");

    let now = Utc::now();
    let time = now.time();
    let date = now.date_naive();
    let file_name = format!("{}\\logs\\{}_{}.log", working_dir.display(), date, time.format("%H-%M-%S"));

    // create a path to the desired file
    let path = Path::new(&file_name);

    // open the path in write-only mode, returns `io::Result<File>`
    match File::create(path) {
      Ok(r) => file.replace(r),
      Err(e) => panic!("couldn't create {}: {}", path.display(), e)
    };
  });
}

fn write_log_file(text: &str) {
  _log_file_callback(&|file| {
    // we don't have a file to write anything in
    if file.is_none() {
      return;
    }

    let t = file.as_mut().unwrap();
    writeln!(t, "{}", text).ok();
  });
}

pub fn log<T: Into<String>>(state: &str, text: T, write_in_file: bool) {
  let s = match state {
    "warning" => "WARNING".bright_yellow(),
    "error" => "ERROR".bright_red(),
    "creating" | "loading" | "successfully" | "starting" => state.to_uppercase().bright_green(),
    _ => "INFO".bright_white()
  };

  let now = Utc::now();
  let time = now.time();
  let date = now.date_naive();
  let log_time = format!("[{} {}]", date, time.format("%H:%M:%S"));
  let t = text.into();
  println!("{} {} {}", log_time, s.bold(), t);
  
  if write_in_file {
    create_log_file();
    write_log_file(&format!("{} {} {}", log_time, state.to_uppercase(), t));
  }
}