use clap::{Parser, Subcommand};
use serde::Deserialize;

use crate::constants::CONFIG_FILE_URL;

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(about = "Show config file")]
    Scf,
    #[command(about = "Edit config file")]
    Ecf,
    #[command(about = "Get config file")]
    Gcf {
        #[clap(short, long, default_value = CONFIG_FILE_URL, help = "File location URL")]
        url: String,
        #[clap(short, long, action, help = "Overwrite current config")]
        overwrite: bool,
    },
    #[command(about = "Extract fields for specific brand")]
    Ext {
        #[clap(short, long, help = "Fields to extract")]
        fields: Vec<String>,
        #[clap(short, long, help = "Brand")]
        brand: String,
        #[clap(short, long, help = "Output file path. If not specified will be use current location")]
        output_file_location: Option<String>,
        #[clap(short, long, action, help = "Verbose info about consuming")]
        verbose: bool,
    },
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub bootstrap_servers: String,
    #[serde(default)]
    pub topic: String,
    #[serde(default)]
    pub schema_registry: String,
    #[serde(default)]
    pub security_protocol: Option<String>,
    #[serde(default)]
    pub security_inter_broker_protocol: Option<String>,
    #[serde(default)]
    pub ssl_enabled_protocols: Option<String>,
    #[serde(default)]
    pub ssl_key_location: Option<String>,
    #[serde(default)]
    pub ssl_key_password: Option<String>,
    #[serde(default)]
    pub ssl_certificate_location: Option<String>,
    #[serde(default)]
    pub ssl_ca_location: Option<String>,
    #[serde(default)]
    pub ssl_endpoint_identification_algorithm: Option<String>,
}
