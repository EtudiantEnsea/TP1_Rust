use pcap::Capture;
use crate::BeaconResult;
use crate::tlv::parse_tlv;

/// Analyse un fichier PCAP et retourne les trames Beacon détectées.
///
/// Cette fonction :
/// - lit les paquets un par un
/// - filtre les trames Beacon
/// - extrait MAC, SSID et DroneID
pub fn analyze_pcap(path: &str) -> Vec<BeaconResult> {
   let mut cap = Capture::from_file(path)
      .expect("Erreur ouverture fichier");

   let mut results = Vec::new();

   while let Ok(packet) = cap.next_packet() {
      let data = packet.data;

      let radiotap_len = get_radiotap_len(data);

      if data.len() < radiotap_len + 24 {
         continue;
      }

      let ieee80211 = &data[radiotap_len..];

      let frame_control =
         u16::from_le_bytes([ieee80211[0], ieee80211[1]]);

      let frame_type = (frame_control >> 2) & 0b11;
      let subtype = (frame_control >> 4) & 0b1111;

      // Beacon uniquement
      if frame_type == 0 && subtype == 8 {
         let src_mac = format_mac(&ieee80211[10..16]);

         let management = &ieee80211[24..];
         if management.len() < 12 {
               continue;
         }

         let tlv = &management[12..];

         let (ssid_opt, is_drone) = parse_tlv(tlv);

         let ssid = ssid_opt.unwrap_or("<hidden>".to_string());

         results.push(BeaconResult {
               mac: src_mac,
               ssid,
               is_drone,
         });
      }
   }

   results
}

/// Calcule la taille de l'en-tête Radiotap
fn get_radiotap_len(data: &[u8]) -> usize {
   if data.len() < 4 {
      return 0;
   }
   u16::from_le_bytes([data[2], data[3]]) as usize
}

/// Convertit une adresse MAC en String lisible
fn format_mac(bytes: &[u8]) -> String {
   bytes.iter()
         .map(|b| format!("{:02x}", b))
         .collect::<Vec<_>>()
         .join(":")
}