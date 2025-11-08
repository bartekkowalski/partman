use std::{ffi::OsStr, fs, path::Path};

use crate::{
    Result, api::digikey, config::Config, data, editor::{Form, edit, edit_file, process_form}
};

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "partman")]
#[command(about = "KiCad DB lib part management", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Cmd
}

#[derive(Subcommand, Debug)]
pub enum Cmd {
    /// Initialize a new partman project
    Init(InitCmd),

    /// Generate SQLite DB file for KiCad
    Build(BuildCmd),
    
    /// Add a new part
    Add(AddCmd),

    /// Resume the last part add
    Resume(ResumeCmd),
}

#[derive(Parser, Debug)]
pub struct InitCmd {}

#[derive(Parser, Debug)]
pub struct BuildCmd {}

#[derive(Parser, Debug)]
pub struct AddCmd {
    // Pre-fill using DigiKey Part Number and API
    #[arg(short, long)]
    pub digikey: Option<String>,

    /// Don't build DB file after adding part
    #[arg(short, long)]
    pub no_build: bool,
}

#[derive(Parser, Debug)]
pub struct ResumeCmd {}

// TODO: SPlit digikey API keys into separate file, create file on init, and reference in partman.toml
impl InitCmd {
    pub fn run(&self) -> Result<()> {
        let config_path = Path::new("partman.toml");
        if config_path.exists() {
            return Err("partman.toml already exists in this directory".into());
        }

        fs::write(config_path, Config::DEFAULT_CONFIG)?;
        println!("Created default partman.toml");

        Ok(())
    }
}

impl BuildCmd {
    pub fn run(&self, config: &Config) -> Result<()> {
        let parts = data::load_all_parts(&config.cat, &config.csv_dir_path)?;
        data::build_kicad_db(&config.db_file_path, parts, &config.cat)?;
        Ok(())
    }
}

impl AddCmd {
    pub fn run(&self, config: &Config) -> Result<()> {
       
        let id = &config.digikey.as_ref().ok_or("digikey config missing")?.client_id;
        let secret = &config.digikey.as_ref().ok_or("digikey config missing")?.client_secret;
        let mut form = match &self.digikey {
            None => Form::default(),
            Some(mpn) => digikey::get_product(id, secret, mpn)?.into(),
        };
        form.categories = Some(config.cat.clone());
        let rslt = edit(&config.editor_cmd, &config.history_dir_path, form)?;

        match rslt {
            None => println!("no changes!"),
            Some(edited_form) => process_form(edited_form, config)?,
        }

        Ok(())
    }
}

impl ResumeCmd {
    pub fn run(&self, config: &Config) -> Result<()> {

        let entries = fs::read_dir(&config.history_dir_path)?;

        let latest_timestamp = entries
            .flatten()
            .filter(|ent| ent.path().extension() == Some(OsStr::new("toml")))
            .filter_map(|ent| {
                ent.path()
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .and_then(|s| s.parse::<u64>().ok())
            })
            .max()
            .ok_or("Error: Could not find temporary file to resume from")?;

        let latest_file = config.history_dir_path
            .join(latest_timestamp.to_string())
            .with_extension("toml");

        println!("Resuming from {}", &latest_file.display());
        
        let rslt = edit_file(&config.editor_cmd, &latest_file)?;

        match rslt {
            None => println!("no changes!"),
            Some(edited_form) => process_form(edited_form, config)?,
        }

        Ok(())
    }
}