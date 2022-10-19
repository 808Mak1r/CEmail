use std::path::PathBuf;

use clap::{Parser};

/// A simple CLI to check if an email exists
#[derive(Parser, Debug, Clone)]
#[clap(version, author, about, long_about = None)]
pub struct Args {
    // #[clap(subcommand)]
    // pub action: Action,

    /// The email to check
    #[clap(short, long, value_parser)]
    pub email: Option<Vec<String>>,

    /// The file containing the list of emails to check
    #[clap(short, long, value_parser)]
    pub file: Option<String>,

    /// The output file
    #[clap(short, long, default_value = "output.json")]
    pub output: PathBuf,
}

// #[derive(Subcommand, Debug, Clone)]
// pub enum Action {
//     /// Check if an email exists
//     Check(CheckArgs),
// }

// #[derive(Parser, Debug, Clone)]
// pub struct CheckArgs {
//     /// The email to check
//     #[clap(short, long, value_parser)]
//     pub email: String,

//     /// The file containing the list of emails to check
//     #[clap(short, long, value_parser)]
//     pub file: Option<String>,
// }
