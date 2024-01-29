use std::{fs, io};
use std::collections::HashSet;
use std::fs::File;
use std::path::PathBuf;

use clap::Parser;
use serde::Deserialize;

const CONFIG_FILE_NAME_TEMPLATE: &str = ".kfe.json";
const CONFIG_FILE_URL: &str = "https://raw.githubusercontent.com/ViacheslavBondarchuk/kafka-field-extractor/master/.kfe.json";

fn main() {
    let arguments: Arguments = Arguments::parse();
    load_config(&arguments.brand);
}

fn get_config_file_path_buf() -> PathBuf {
    let mut config_dir_path_buf = dirs::config_dir().expect("Ca not get configuration directory");
    config_dir_path_buf.push(CONFIG_FILE_NAME_TEMPLATE);
    config_dir_path_buf
}

fn download_config(store_path: &PathBuf) {
    println!("Downloading config file....");
    let body = reqwest::blocking::get(CONFIG_FILE_URL).expect("Can not download config")
        .text()
        .expect("Can not extract body");
    let mut config_file = File::create(store_path).expect("Can not create config file");
    io::copy(&mut body.as_bytes(), &mut config_file).expect("Can not store file");
}

fn load_config(brand: &String) {
    let config_path_buf = get_config_file_path_buf();
    if !config_path_buf.exists() {
        download_config(&config_path_buf);
    }

    let configs: Vec<Config> = serde_json::from_slice(&fs::read(&config_path_buf).expect("Can not read config"))
        .expect("Can not deserialize config");
    println!("Loading config for brand: {} by path: {}", brand, &config_path_buf.display());
}


#[derive(Parser, Debug)]
#[command(author, version)]
struct Arguments {
    #[arg(short, long)]
    brand: String,
}

#[derive(Debug, Deserialize)]
enum RecordType {
    Avro,
    Json,
}

#[derive(Debug, Deserialize)]
struct Config {
    brand: String,
    fields: HashSet<String>,
    file_location: String,
    record_type: RecordType,
}
