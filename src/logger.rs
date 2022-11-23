/**
 * THIS IS JUST A SIMPLE CONSOLE LOGGER!
 */

pub fn log<T: Into<String>>(state: &str, text: T) {
  use colored::*;

  let s = match state {
    "warning" => "WARNING".bright_yellow(),
    "error" => "ERROR".bright_red(),
    "creating" | "loading" | "successfully" | "starting" => state.to_uppercase().bright_green(),
    _ => "INFO".bright_white()
  };

  println!("{} {}", s.bold(), text.into());
}