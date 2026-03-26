///Indique à Cargo où trouver les bibliothèques natives Npcap sous Windows.
/// Nécessaire pour le linking avec `wpcap.lib`.

fn main() {
   println!("cargo:rustc-link-search=native=C:\\Users\\loick\\OneDrive\\Bureau\\Ecole\\Rust\\npcap\\Lib\\x64");
}