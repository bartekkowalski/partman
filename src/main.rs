use std::time::Instant;
use clap::Parser;

mod config;
mod data;
mod cli;
mod part;
mod editor;
mod api;
mod util;

use cli::{Cli, Cmd};
use config::Config;


pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = core::result::Result<T, Error>;

fn main() -> Result<()> {
    let start_time = Instant::now();

    match Cli::parse().command {
        Cmd::Init(subcmd) => subcmd.run()?,
        Cmd::Add(subcmd) => {
            let config = Config::find_and_load()?;
            subcmd.run(&config)?
        },
        Cmd::Build(subcmd) => {
            let config = Config::find_and_load()?;
            subcmd.run(&config)?
        },
        Cmd::Resume(subcmd) => {
            let config = Config::find_and_load()?;
            subcmd.run(&config)?
        },
    };

    let elapsed = start_time.elapsed();
    println!("  Time elapsed:        {:.2?}", elapsed);

    Ok(())
}
