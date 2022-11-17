use std::fs;

use toml;

mod structs;

static mut CONFIG: Option<structs::Data> = None;
pub fn get_config() -> &'static structs::Data {
  unsafe {
    CONFIG.as_ref().unwrap()
  }
}

pub fn load_config(file_path: &str) {
  let file_content = get_data_from_file(&file_path);

  unsafe {
    CONFIG = match toml::from_str(&file_content.as_str()) {
      Ok(r) => Some(r),
      Err(e) => {
        super::logger::log("error", format!("Unable to load data from `{}`\n{}", &file_path, e.to_string()).as_str());
        std::process::exit(1);
      }
    }
  }
}

fn get_data_from_file(file_path: &str) -> String {
  super::logger::log("loading", format!("`{}`", &file_path).as_str());

  match fs::read_to_string(&file_path) {
    Ok(r) => r,
    Err(e) => {
      super::logger::log("error", format!("could not read file `{}`\n{}", &file_path, e.to_string()).as_str());
      std::process::exit(1);
    }
  }
}