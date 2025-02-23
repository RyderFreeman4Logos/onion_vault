use crate::config::default::PROJECTDIR;


use clap::{Parser, Subcommand};



#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Sets a custom BIP32 PATH
    #[arg(short, long, default_value = "m/44h/60h/11h/0/12")]
    pub bip32_path: String,

    /// If the value is true, you need to use the mouse wheel
    /// after each Enter key press
    /// for the history of interactions.
    #[arg(long, default_value = "false")]
    pub auto_clear: bool,

    /// Sets a custom config file
    #[arg(short, long, default_value_t = PROJECTDIR.config_file())]
    pub config_file: String,

    #[arg(short, long, default_value_t = PROJECTDIR.data_dir())]
    pub data_dir: String,

    #[command(subcommand)]
    pub command: Option<Commands>,
}




#[derive(Subcommand)]
pub enum Commands {
    /// does testing things
    Man {
        /// lists all possible ERRORs and solution
        #[arg(long)]
        errors: bool,
    },
}


