pub mod common;
pub mod re_export;
pub mod config;
mod tests;
mod help;
mod args;
mod app;

mod operations;



use crate::args::{
    Commands,
    Cli,
};

use clap::Parser;

pub fn run() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Man { errors: _ }) => {
            crate::help::print_errs_and_solution()
        }
        None => app::AppBuilder::default()
            .bip32_path(&cli.bip32_path)
            .data_dir(&cli.data_dir)
            .auto_clear(cli.auto_clear)
            .build()
            .unwrap()
            .main_loop()
    }
}
