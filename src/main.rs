use clap::{Parser, Subcommand};
mod domain;
use domain::dictionary::Dictionary;
mod infrastructure;
use infrastructure::repository::{DictionaryJsonRepository, IDictionaryRepository};
mod application;
use application::dictionary_application_service::DictionaryApplicationService;

#[derive(Parser, Debug)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    arg_required_else_help = true,
)]
struct Cli {
    #[clap(subcommand)]
    subcommand: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    #[clap(about = "Initialize new dictionary")]
    Init,

    #[clap(about = "Show keys from dictionary")]
    Show,

    #[clap(arg_required_else_help = true, about = "Add new dictionary")]
    Add {
        key: String,
        value: String,
        #[clap(default_value = "")]
        description: String,
    },

    #[clap(arg_required_else_help = true, about = "Get value from dictionary")]
    Get { key: String },

    #[clap(arg_required_else_help = true, about = "Remove value from dictionary")]
    Rm { key: String },
}

fn main() {
    let dictionary_application_service =
        DictionaryApplicationService::new(DictionaryJsonRepository::new());

    let cli = Cli::parse();

    // dictionary_application_service.init().unwrap();

    match cli.subcommand {
        SubCommands::Init => dictionary_application_service.init().unwrap(),
        SubCommands::Add {
            key,
            value,
            description,
        } => dictionary_application_service
            .register(Dictionary::new(key, value, description))
            .unwrap(),
        SubCommands::Show => dictionary_application_service.show(),
        SubCommands::Get { key } => {
            println!("{}", dictionary_application_service.get(key).unwrap())
        }
        SubCommands::Rm { key } => {
            let dictionaries = dictionary_application_service.remove(key);
            dictionary_application_service.save(dictionaries).unwrap();
            dictionary_application_service.show();
        }
    }
}
