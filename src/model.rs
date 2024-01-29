use clap::{Parser, Subcommand};
use serde::Deserialize;

use crate::constants::CONFIG_FILE_URL;

#[derive(Subcommand, Debug)]
pub enum Command {
    ShowConfig,
    GetConfigFile {
        #[clap(short, long, default_value = CONFIG_FILE_URL)]
        url: String,
        #[clap(short, long, action)]
        overwrite: bool,
    },
    Extract {
        #[clap(short, long)]
        fields: Vec<String>,
        #[clap(short, long)]
        brand: String,
        #[clap(short, long)]
        output_file_location: Option<String>,
    },
}

#[derive(Parser, Debug)]
#[command(propagate_version = true)]
#[command(author, version, about, long_about = None)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    bootstrap_servers: String,
    topic: String,
    max_poll_records: i32,
    request_timeout_ms: i64,
    #[serde(default)]
    security_protocol: Option<String>,
    #[serde(default)]
    security_inter_broker_protocol: Option<String>,
    #[serde(default)]
    ssl_enabled_protocols: Option<Vec<String>>,
    #[serde(default)]
    ssl_key_password: Option<String>,
    #[serde(default)]
    ssl_endpoint_identification_algorithm: Option<String>,
}