use clap::{Parser, ValueEnum};
use tp::parser::analyze_pcap;
use tp::output::{save_json, save_csv};


#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli{

    #[arg(long, conflicts_with = "interface")]
    pcap: Option<String>,

    #[arg(long, conflicts_with = "pcap")]
    interface: Option<String>,

    #[arg(long)]
    cards: bool,

    #[arg(long)]
    filter: Option<String>,
    
    #[arg(long, default_value_t = 10)]
    packet_count: u32,


    #[arg(long, value_enum, default_value_t = Format::Json)]    
    output_format: Format,
    
    #[arg(long, default_value = "results.json")]    
    output_file: String

}

#[derive(ValueEnum, Clone)]
enum Format {
    Json,
    Csv,
}

fn main() {
    let args = Cli::parse();

    let path = args.pcap.expect("Fichier PCAP requis");
    let results = analyze_pcap(&path);

    match args.output_format {
        Format::Json => save_json(&args.output_file, &results),
        Format::Csv => save_csv(&args.output_file, &results),
    }
}