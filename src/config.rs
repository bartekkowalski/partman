use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::PathBuf};
use crate::Result;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Digikey {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Secrets {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digikey: Option<Digikey>,
}

impl Secrets {
    pub const DEFAULT_SECRETS: &str = r#"
# Partman API Keys File

# Digikey API credentials, get them from https://developer.digikey.com/
[digikey]
client_id = ""
client_secret = ""
"#;
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Cat {
    pub code: String,
    pub filename: String,
    pub subcategories: Vec<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub editor_cmd: String,
    pub db_file_path: PathBuf,
    pub csv_dir_path: PathBuf,
    pub history_dir_path: PathBuf,
    #[serde(skip)]
    pub digikey: Option<Digikey>,
    pub cat: HashMap<String, Cat>
}

impl Config {
    const CONFIG_NAME: &str = "partman.toml";
    const SECRETS_NAME: &str = "secrets.toml";

    pub fn find_and_load() -> Result<Self> {
        let mut current = std::env::current_dir()?;

        let root = loop {
            let path = current.join(Self::CONFIG_NAME);
            if path.exists() {
                break current;
            }

            match current.parent() {
                Some(parent) => current = parent.to_path_buf(),
                None => return Err(format!(
                    "Could not find {} in current directory or any parent directory",
                    Self::CONFIG_NAME
                )
                .into())
            }
        };
        
        let config_path = root.join(Self::CONFIG_NAME);
        let conf_str = fs::read_to_string(&config_path)?;
        let mut config: Self = toml::from_str(&conf_str)?;

        let secrets_path = root.join(Self::SECRETS_NAME);
        if secrets_path.exists() {
            let secrets_str = fs::read_to_string(&secrets_path)?;
            if !secrets_str.trim().is_empty() {
                match toml::from_str::<Secrets>(&secrets_str) {
                    Ok(secrets) => config.digikey = secrets.digikey,
                    Err(e) => return Err(format!("Failed to parse secrets.toml: {}", e).into()),
                }
            }
        }
        
        // Allow for paths relative to partman.toml
        if config.csv_dir_path.is_relative() {
            config.csv_dir_path = root.join(config.csv_dir_path.clone());
        };

        // Allow for paths relative to partman.toml
        if config.db_file_path.is_relative() {
            config.db_file_path = root.join(config.db_file_path.clone());
        };

        // Allow for paths relative to partman.toml
        if config.history_dir_path.is_relative() {
            config.history_dir_path = root.join(config.history_dir_path.clone());
        };


        Ok(config)
    }

    pub const DEFAULT_CONFIG: &str = r#"
# Partman Config File

# Command to run your editor
editor_cmd = "code.cmd --wait --new-window" # VSCode
# editor_cmd = "nvim" # Neovim

# Path to the database file
# Can be absolute or relative to partman.toml
db_file_path = "build/components.db"

# Path to the directory containing the CSV files for each category
# Can be absolute or relative to partman.toml
csv_dir_path = "csv/"

# Path to the directory where partman will store temporary files
# Can be absolute or relative to partman.toml
history_dir_path = "history/"

# Categories, each category has a code, a file and a list of subcategories
[cat.resistor]
code = "RES"
filename = "resistor.csv"
subcategories = ["chip", "through-hole", "potentiometer"]

[cat.capacitor]
code = "CAP"
filename = "capacitor.csv"
subcategories = ["chip", "aluminium"]
"#;
}