use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use clap::Parser;

use crate::constants::*;
use crate::model::{Arguments, Command, Config};

mod model;
mod constants;

fn main() -> () {
    match Arguments::parse().command {
        Command::Extract {
            brand,
            fields,
            output_file_location
        } => handle_extract_command(brand, fields, output_file_location),
        Command::GetConfigFile {
            url,
            overwrite
        } => handle_get_config_command(url, overwrite),
        Command::ShowConfig => handle_show_config(),
    }
}

fn get_config_directory_path() -> PathBuf {
    let mut config_dir_path_buf = dirs::config_dir().expect("Can not get config directory");
    config_dir_path_buf.push(CONFIG_FILE_NAME_TEMPLATE);
    config_dir_path_buf
}

fn download_config(url: &str) -> String {
    println!("Downloading configuration from {}", url);
    let response = reqwest::blocking::get(url).expect("Can not download config");
    response.text().expect("Can not extract body")
}

fn store_config_file(url: &String, config_directory_path: &PathBuf) {
    let mut config_file = File::create(&config_directory_path).expect("Can not create config file");
    config_file.write_all(&download_config(&url).as_bytes()).expect("Can not write config file");
}

fn load_config() -> HashMap<String, Config> {
    let config_directory_path = get_config_directory_path();
    let bytes = fs::read(config_directory_path).expect("Can not read config");
    serde_json::from_slice(&bytes).expect("Can not deserialize config")
}

fn handle_get_config_command(url: String, overwrite: bool) {
    let config_directory_path = get_config_directory_path();
    if config_directory_path.exists() && !overwrite {
        panic!("Config exists. For detail information use --help")
    } else if config_directory_path.exists() && overwrite {
        println!("Overwriting existing config in {}", &config_directory_path.display());
        fs::remove_file(&config_directory_path).expect("Can not delete config");
        store_config_file(&url, &config_directory_path);
    } else {
        store_config_file(&url, &config_directory_path);
    }
}

fn handle_extract_command(brand: String, fields: Vec<String>, output_file_location: Option<String>) {
    print!("Extract")
}

fn handle_show_config() {
    println!("{:?}", load_config())
}







