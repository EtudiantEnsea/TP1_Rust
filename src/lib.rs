use serde::Serialize;

/// Modules publics de la bibliothèque
pub mod parser;
pub mod output;
mod tlv;

/// Structure représentant un réseau détecté via une trame Beacon
#[derive(Serialize)]
pub struct BeaconResult {
   
   pub mac: String,
   pub ssid: String,
   pub is_drone: bool,
}