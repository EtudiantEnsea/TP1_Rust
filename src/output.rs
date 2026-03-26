use crate::BeaconResult;

/// Sauvegarde les résultats au format JSON
pub fn save_json(path: &str, data: &Vec<BeaconResult>) {
   let json = serde_json::to_string_pretty(data)
      .expect("Erreur JSON");

   std::fs::write(path, json)
      .expect("Erreur écriture fichier");
}

/// Sauvegarde les résultats au format CSV
pub fn save_csv(path: &str, data: &Vec<BeaconResult>) {
   let mut wtr = csv::Writer::from_path(path)
      .expect("Erreur CSV");

   for item in data {
      wtr.serialize(item).expect("Erreur serialize");
   }

   wtr.flush().expect("Erreur flush");
}