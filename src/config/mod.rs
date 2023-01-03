use std::{
  env,
  fs
};

mod structs;

pub fn load_config() -> structs::Data {
  let working_dir = env::current_dir().expect("can't access current dir");
  let file_path = format!("{}\\config.toml", working_dir.display());
  let file_content = _get_data_from_file(&file_path);

  match toml::from_str::<structs::Data>(&file_content) {
    Ok(r) => r,
    Err(e) => {
      crate::logger::log("error", format!("Unable to load data from `{}`\n{}", file_path, e));
      std::process::exit(1);
    }
  }
}

fn _get_data_from_file(file_path: &str) -> String {
  crate::logger::log("loading", format!("`{}`", file_path));

  match fs::read_to_string(file_path) {
    Ok(r) => r,
    Err(e) => {
      crate::logger::log("error", format!("could not read file `{}`\n{}", file_path, e));
      std::process::exit(1);
    }
  }
}