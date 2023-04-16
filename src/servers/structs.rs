use serde::{
  Serialize,
  Deserialize
};

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct Server {
  pub address: String,
  pub port: u16,
  pub name: String,
  pub version: String,
  pub players: u16,
  #[serde(rename = "maxPlayers")]
  pub max_players: u16,
  pub country: String,
  pub description: String,
  pub website: String,
  #[serde(rename = "gameMode")]
  pub game_mode: String,
  pub language: String,
  #[serde(rename = "useP2P")]
  pub use_p2p: bool,
  #[serde(rename = "useZT")]
  pub use_zt: bool,
  #[serde(rename = "ztID")]
  pub zt_id: String,
  #[serde(rename = "ztAddress")]
  pub zt_address: String,
  #[serde(rename = "publicKeyModulus")]
  pub public_key_modulus: String,
  #[serde(rename = "publicKeyExponent")]
  pub public_key_exponent: String,
  // we add these following fields ourselves
  #[serde(skip)]
  pub last_update: u64
}

impl Server {
  /// Replace all bad words.
  pub fn filter_bad_words(&mut self) {
    let censor = censor::Standard + censor::Sex;
    self.name = censor.censor(&self.name);
    self.description = censor.censor(&self.description);
    self.website = censor.censor(&self.website);
  }
}