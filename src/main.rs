use std::collections::HashSet;
use std::path::PathBuf;

use clap::Parser;
use serde::Deserialize;

const CONFIG_FILE_NAME_TEMPLATE: &str = ".kfe.json";\

enum RecordType {
    Avro,
    Json,
}

fn main() {
    let arguments: Arguments = Arguments::parse();
    load_config(&arguments.brand.unwrap_or("".to_string()));
}

fn get_config_file_path_buf() -> PathBuf {
    let mut config_dir_path_buf = dirs::config_dir().expect("Ca not get configuration directory");
    config_dir_path_buf.push(CONFIG_FILE_NAME_TEMPLATE);
    config_dir_path_buf
}

fn load_config(brand: &String) -> () {
    let config_path_buf = get_config_file_path_buf();
    println!("Loading config for brand: {} by path: {}", brand, config_path_buf.display());
}


#[derive(Parser, Debug)]
#[command(author, version)]
struct Arguments {
    #[arg(short, long)]
    brand: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Config {
    brand: String,
    fields: HashSet<String>,
    file_location: String,
    record_type: RecordType,
}
