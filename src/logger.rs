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
use chrono::{
  Utc,
  NaiveTime,
  NaiveDate
};
use colored::*;

static mut LOG_FILE: Mutex<Option<File>> = Mutex::new(None);
fn _log_file_callback(callback: &dyn Fn(&mut MutexGuard<Option<File>>)) {
  if let Ok(mut r) = unsafe { LOG_FILE.lock() } {
    callback(&mut r)
  }
}

fn _create_log_file(time: &NaiveTime, date: &NaiveDate) {
  _log_file_callback(&|file| {
    // we already have a file
    if file.is_some() { return; }

    // Create a "logs" folder in the current directory if none with that name exists
    let working_dir = env::current_dir().expect("can't access current dir");
    let log_path = format!("{}\\logs", working_dir.display());
    fs::create_dir_all(&log_path).expect("couldn't create `logs` dir");

    // create a path to the desired file
    let file_name = format!("{}\\{}_{}.log", log_path, date, time.format("%H-%M-%S"));
    let path = Path::new(&file_name);

    // open the path in write-only mode
    match File::create(path) {
      Ok(r) => file.replace(r),
      Err(e) => panic!("couldn't create {}: {}", path.display(), e)
    };
  });
}

fn _write_log_file(text: &str) {
  _log_file_callback(&|file| {
    // we don't have a file to write anything in
    if file.is_none() { return; }

    let t = file.as_mut().unwrap();
    writeln!(t, "{}", text).ok();
  });
}

pub fn log<T: Into<String>>(state: &str, text: T) {
  let now = Utc::now();
  let time = now.time();
  let date = now.date_naive();

  _create_log_file(&time, &date);

  let s = match state {
    "warning" => "WARNING".bright_yellow(),
    "error" => "ERROR".bright_red(),
    "creating" | "loading" | "successfully" | "starting" => state.to_uppercase().bright_green(),
    _ => "INFO".bright_white()
  };
  
  let t = text.into();
  let log_time = format!("[{} {}]", date, time.format("%H:%M:%S"));
  println!("{} {} {}", log_time, s.bold(), t);
  
  _write_log_file(&format!("{} {} {}", log_time, state.to_uppercase(), t));
}