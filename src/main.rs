use clap::{Parser, Subcommand};
use dirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

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
    #[clap(about = "Show keys from dictionary")]
    Init,

    #[clap(about = "Show keys from dictionary")]
    List,

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

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Dictionary {
    key: String,
    value: String,
    description: String,
}

impl Dictionary {
    fn new(key: String, value: String, description: String) -> Self {
        Dictionary {
            key: key,
            value: value,
            description: description,
        }
    }
}

trait IDictionaryRepository {
    fn init(&self) -> Result<(), String>;

    fn get_home_path(&self) -> PathBuf {
        let home_dir = dirs::home_dir().unwrap();
        Path::new(&home_dir).join(".wordic")
    }

    /// Register data
    fn register(&self, dictionary: Dictionary) -> Result<(), String>;
    fn load_data(&self) -> Vec<Dictionary>;
    fn save(&self, dictionaries: Vec<Dictionary>) -> Result<(), String>;
    fn get(&self, key: String) -> Result<String, String>;
    fn remove(&self, key: String) -> Vec<Dictionary>;
}

struct DictionaryJsonRepository {
    file_name: String,
}
impl IDictionaryRepository for DictionaryJsonRepository {
    fn init(&self) -> Result<(), String> {
        let save_dir = self.get_home_path();
        let file_path = save_dir.join(&self.file_name);
        if !save_dir.exists() {
            match fs::create_dir(save_dir) {
                Err(why) => println!("! {:?}", why.kind()),
                Ok(_) => {}
            }
        }
        if !file_path.exists() {
            self.save(vec![]).unwrap();
            println!("Created {}", file_path.display());
        } else {
            println!("Already created. {}", file_path.display());
        }
        Ok(())
    }

    /// Register data
    fn register(&self, dictionary: Dictionary) -> Result<(), String> {
        let key = dictionary.key.clone();
        let mut dictionaries = self.remove(key);
        dictionaries.push(dictionary);
        self.save(dictionaries).unwrap();
        Ok(())
    }

    fn load_data(&self) -> Vec<Dictionary> {
        let save_dir = self.get_home_path();
        let file_path = save_dir.join(&self.file_name);
        let content = fs::read_to_string(file_path).expect("Failed to load JSON");
        let deserialized: Vec<Dictionary> = serde_json::from_str(&content).unwrap();
        deserialized
    }

    fn save(&self, dictionaries: Vec<Dictionary>) -> Result<(), String> {
        let save_dir = self.get_home_path();
        let file_path = save_dir.join(&self.file_name);
        let serialized = serde_json::to_string(&dictionaries).unwrap();
        let mut file = fs::File::create(file_path).unwrap();
        let _ = file.write_all(serialized.as_bytes()).unwrap();
        Ok(())
    }

    fn get(&self, key: String) -> Result<String, String> {
        let mut value: String = "".to_string();
        let dictionaries = self.load_data();
        for i in dictionaries {
            if i.key == key {
                value = i.value;
            }
        }
        Ok(value)
    }

    fn remove(&self, key: String) -> Vec<Dictionary> {
        let mut dictionaries: Vec<Dictionary> = self.load_data();
        dictionaries.retain(|x| x.key != key);
        dictionaries
    }
}

struct DictionaryApplicationService<T> {
    dictionary_repository: T,
}

impl<T: IDictionaryRepository> DictionaryApplicationService<T> {
    fn init(&self) -> Result<(), String> {
        self.dictionary_repository.init()
    }

    fn new(dictionary_repository: T) -> Self {
        DictionaryApplicationService {
            dictionary_repository: dictionary_repository,
        }
    }

    fn show(&self) {
        let dictionaries = self.dictionary_repository.load_data();
        for (i, v) in dictionaries.iter().enumerate() {
            if v.description.len() == 0 {
                println!("{} : {}", i, v.key);
            } else {
                println!("{} : {} <{}> ", i, v.key, v.description);
            }
        }
    }

    fn register(&self, dictionary: Dictionary) -> Result<(), String> {
        self.dictionary_repository.register(dictionary)
    }

    fn save(&self, dictionaries: Vec<Dictionary>) -> Result<(), String> {
        self.dictionary_repository.save(dictionaries)
    }

    fn get(&self, key: String) -> Result<String, String> {
        self.dictionary_repository.get(key)
    }

    fn remove(&self, key: String) -> Vec<Dictionary> {
        self.dictionary_repository.remove(key)
    }
}

fn main() {
    let dictionary_application_service =
        DictionaryApplicationService::new(DictionaryJsonRepository {
            file_name: "wordic.json".to_string(),
        });

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
        SubCommands::List => dictionary_application_service.show(),
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_register() {
        let dictionary_repository = DictionaryJsonRepository {
            file_name: "wordic.json".to_string(),
        };
        let value = dictionary_repository
            .register(Dictionary::new(
                String::from("sample"),
                String::from("sample_value"),
                String::from(""),
            ))
            .unwrap();
        assert!(() == value);
    }

    #[test]
    fn test_get() {
        let dictionary_repository = DictionaryJsonRepository {
            file_name: "wordic.json".to_string(),
        };
        let value = dictionary_repository
            .register(Dictionary::new(
                String::from("sample"),
                String::from("sample_value"),
                String::from(""),
            ))
            .unwrap();
        let value = dictionary_repository.get(String::from("sample")).unwrap();
        assert!(value == "sample_value");
    }
}
