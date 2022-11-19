use std::fs;

mod structs;

pub fn load_config() -> structs::Data {
  let file_path = "./config.toml";
  let file_content = _get_data_from_file(&file_path);

  match toml::from_str::<structs::Data>(&file_content.as_str()) {
    Ok(r) => r,
    Err(e) => {
      super::logger::log("error", format!("Unable to load data from `{}`\n{}", &file_path, e.to_string()));
      std::process::exit(1);
    }
  }
}

fn _get_data_from_file(file_path: &str) -> String {
  super::logger::log("loading", format!("`{}`", &file_path));

  match fs::read_to_string(&file_path) {
    Ok(r) => r,
    Err(e) => {
      super::logger::log("error", format!("could not read file `{}`\n{}", &file_path, e.to_string()));
      std::process::exit(1);
    }
  }
}