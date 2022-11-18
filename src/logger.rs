/**
 * THIS IS JUST A SIMPLE CONSOLE LOGGER!
 */

pub fn log(state: &str, text: &str) {
  use colored::*;

  let s = match state {
    "warning" => "WARNING".bold().bright_yellow(),
    "error" => "ERROR".bold().bright_red(),
    "creating" | "loading" | "successfully" | "starting" => state.to_uppercase().bold().bright_green(),
    _ => "INFO".bold()
  };

  println!("{} {}", s, text);
}