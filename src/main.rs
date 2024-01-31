use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use clap::Parser;

use crate::constants::{CONFIG_FILE_NAME_TEMPLATE, FOR_ADDITIONAL_INFO_MESSAGE};
use crate::kafka::StreamKafkaConsumer;
use crate::model::{Arguments, Command, Config};

mod model;
mod constants;
mod kafka;

fn main() -> () {
    match Arguments::parse().command {
        Command::Ext {
            brand,
            fields,
            output_file_location,
            verbose
        } => handle_extract_command(brand, fields, output_file_location, verbose),
        Command::Gcf {
            url,
            overwrite
        } => handle_get_config_command(url, overwrite),
        Command::Scf => handle_show_config(),
        Command::Ecf => handle_edit_config(),
    }
}

fn get_config_file_path() -> PathBuf {
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
    config_file.sync_all().expect("Can not sync all config data");
}

fn load_config() -> HashMap<String, Config> {
    let config_file_path = get_config_file_path();
    if !config_file_path.exists() {
        panic!("Config file is not exists. {}", FOR_ADDITIONAL_INFO_MESSAGE)
    }
    let bytes: Vec<u8> = fs::read(config_file_path).expect("Can not read config");
    serde_json::from_slice(&bytes).expect("Can not deserialize config")
}

fn handle_get_config_command(url: String, overwrite: bool) {
    let config_file_path = get_config_file_path();
    if config_file_path.exists() && !overwrite {
        println!("Config exists. {}", FOR_ADDITIONAL_INFO_MESSAGE)
    } else if config_file_path.exists() && overwrite {
        println!("Overwriting existing config in {}", &config_file_path.display());
        store_config_file(&url, &config_file_path);
    } else {
        store_config_file(&url, &config_file_path);
    }
}

fn handle_extract_command(brand: String, fields: Vec<String>, output_file_location: Option<String>, verbose: bool) {
    let configs = load_config();
    match configs.get(&brand) {
        Some(config) => {
            let _ = StreamKafkaConsumer::new(&config);
        }
        None => println!("Config for brand {} does not exists", &brand)
    }
}

fn handle_show_config() {
    println!("{:#?}", load_config())
}

fn handle_edit_config() {
    let config_file_path = get_config_file_path();
    if !config_file_path.exists() {
        println!("Config does not exists. Location: {}", &config_file_path.display())
    } else {
        println!("Opening config file {}", &config_file_path.display());
        open::that(config_file_path).expect("Can not edit config file")
    }
}







