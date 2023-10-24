mod rdict;
use clap::{Args, Parser, Subcommand};
use rdict::{download_json::download_json, json_to_db::json_to_db, search_word::search_word};

// Define the command-line arguments
#[derive(Parser)]
#[command(name = "rdict")]
#[command(author = "Lodobo. <lodobo.n8qbt@simplelogin.com>")]
#[command(version = "1.0")]
#[command(about = "Offline CLI dictionary")]
struct Cli {
    /// Search word
    #[arg(short, long)]
    word: Option<String>,

    #[command(subcommand)]
    subcommand: Option<SubCommands>,
}

#[derive(Subcommand)]
enum SubCommands {
    /// Download json file from kaikki.org
    DownloadJson,
    /// Parse json file and create database
    JsonToDB,
    /// Search word in dictionary
    Search(Search),
}

#[derive(Args)]
struct Search {
    word: String,
}

fn main() {
    let cli = Cli::parse();

    match cli.subcommand {
        Some(SubCommands::DownloadJson) => {
            download_json();
        }
        Some(SubCommands::JsonToDB) => {
            json_to_db();
        }
        Some(SubCommands::Search(arg)) => {
            search_word(&arg.word);
        }
        None => {
            if let Some(query_word) = cli.word {
                search_word(&query_word);
            } else {
                println!("No Arguments provided");
            }
        }
    }
}
