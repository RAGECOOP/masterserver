use serde::{
  Serialize,
  Deserialize
};

#[derive(Serialize, Deserialize, Clone)]
pub struct Server {
  pub address: String,
  pub port: u16,
  pub name: String,
  pub version: String,
  pub players: u16,
  #[serde(rename = "maxPlayers")]
  pub max_players: u16,
  pub country: Option<String>,
  pub description: Option<String>,
  pub website: Option<String>,
  #[serde(rename = "gameMode")]
  pub game_mode: Option<String>,
  pub language: Option<String>,
  #[serde(rename = "useP2P")]
  pub use_p2p: bool,
  #[serde(rename = "useZT")]
  pub use_zt: bool,
  #[serde(rename = "ztID")]
  pub zt_id: Option<String>,
  #[serde(rename = "ztAddress")]
  pub zt_address: Option<String>,
  #[serde(rename = "publicKeyModulus")]
  pub public_key_modulus: String,
  #[serde(rename = "publicKeyExponent")]
  pub public_key_exponent: String,
  // We add these following fields ourselves
  #[serde(rename = "playerStats", skip_deserializing)]
  pub player_stats: PlayerStats,
  #[serde(skip)]
  pub last_update: u64
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct PlayerStats {
  pub players: Vec<u16>,
  #[serde(skip)]
  pub last_update: u64
}