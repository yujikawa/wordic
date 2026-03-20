use clap::{Parser, Subcommand};
mod domain;
use domain::dictionary::Dictionary;
mod infrastructure;
use infrastructure::repository::{DictionaryJsonRepository, IDictionaryRepository};
mod application;
use application::dictionary_application_service::DictionaryApplicationService;
mod functions;
use functions::display::{display, Color};

#[derive(Parser, Debug)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    arg_required_else_help = true,
)]
struct Cli {
    #[command(subcommand)]
    subcommand: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    #[command(about = "Initialize new dictionary")]
    Init,

    #[command(about = "Show all words in dictionary")]
    Show,

    #[command(arg_required_else_help = true, about = "Add a new word")]
    Add {
        key: String,
        value: String,
        #[arg(default_value = "")]
        description: String,
    },

    #[command(arg_required_else_help = true, about = "Get value for a key")]
    Get {
        key: String,
        /// Show full entry including description and timestamp
        #[arg(short, long)]
        full: bool,
    },

    #[command(arg_required_else_help = true, about = "Remove a word from dictionary")]
    Rm { key: String },

    #[command(
        arg_required_else_help = true,
        about = "Search words by keyword (key/value/description)"
    )]
    Search { keyword: String },

    #[command(
        arg_required_else_help = true,
        about = "Export dictionary to a JSON file"
    )]
    Export { path: String },

    #[command(
        arg_required_else_help = true,
        about = "Import dictionary from a JSON file (replaces current data)"
    )]
    Import { path: String },
}

fn main() {
    let svc = DictionaryApplicationService::new(DictionaryJsonRepository::new());
    let cli = Cli::parse();

    match cli.subcommand {
        SubCommands::Init => svc.init().unwrap(),
        SubCommands::Show => svc.show(),
        SubCommands::Add {
            key,
            value,
            description,
        } => svc
            .register(Dictionary::new(key, value, description))
            .unwrap(),
        SubCommands::Get { key, full } => {
            if full {
                svc.get_full(key);
            } else {
                let value = svc.get(key).unwrap();
                if value.is_empty() {
                    display("Not found in your dictionary.", Color::Red);
                } else {
                    println!("{}", value);
                }
            }
        }
        SubCommands::Rm { key } => {
            let dictionaries = svc.remove(key);
            svc.save(dictionaries).unwrap();
            svc.show();
        }
        SubCommands::Search { keyword } => svc.search(keyword),
        SubCommands::Export { path } => svc.export(path).unwrap(),
        SubCommands::Import { path } => svc.import(path).unwrap(),
    }
}
