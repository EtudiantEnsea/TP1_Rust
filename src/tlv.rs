/// Analyse les champs TLV d'une trame Beacon.
///
/// Retourne :
/// - le SSID (si présent)
/// - un booléen indiquant la présence d'un DroneID
pub fn parse_tlv(data: &[u8]) -> (Option<String>, bool) {
    let mut i = 0;
    let mut ssid = None;
    let mut is_drone = false;

    while i + 2 <= data.len() {
        let tag = data[i];
        let len = data[i + 1] as usize;

        if i + 2 + len > data.len() {
            break;
        }

        let value = &data[i + 2..i + 2 + len];

        match tag {
            0x00 => {
                ssid = Some(String::from_utf8_lossy(value).to_string());
            }
            0xdd => {
               is_drone = true;
            }
            _ => {}
        }

        i += 2 + len;
    }

    (ssid, is_drone)
}