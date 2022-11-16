use super::super::domain::dictionary::Dictionary;
use super::super::functions::display::{display, Color};
use super::super::infrastructure::repository::IDictionaryRepository;

pub struct DictionaryApplicationService<T> {
    dictionary_repository: T,
}

impl<T: IDictionaryRepository> DictionaryApplicationService<T> {
    pub fn init(&self) -> Result<(), String> {
        self.dictionary_repository.init()
    }

    pub fn new(dictionary_repository: T) -> Self {
        DictionaryApplicationService {
            dictionary_repository: dictionary_repository,
        }
    }

    pub fn show(&self) {
        let dictionaries = self.dictionary_repository.load_data();
        let len = dictionaries.len();
        if len > 0 {
            display(
                &format!("You have created {} dictionaries", len),
                Color::Green,
            );
            println!();
            println!(
                "{0: <10} | {1: <10} | {2: <30} | {3: <35}",
                "No", "Key", "Description", "Created at"
            );
            println!(
                "{0:-<10} | {1:-<10} | {2:-<30} | {3:-<35}",
                "-", "-", "-", "-"
            );

            for (i, v) in dictionaries.iter().enumerate() {
                if v.description.len() == 0 {
                    println!(
                        "{0: <10} | {1: <10} | {2: <30} | {3: <35}",
                        i, v.key, "", v.timestamp
                    );
                } else {
                    if v.description.len() > 30 {
                        let description = v.description[..28].to_string() + "..";
                        println!(
                            "{0: <10} | {1: <10} | {2: <30} | {3: <35}",
                            i, v.key, description, v.timestamp
                        );
                    } else {
                        println!(
                            "{0: <10} | {1: <10} | {2: <30} | {3: <35}",
                            i, v.key, v.description, v.timestamp
                        );
                    }
                }
            }
        } else {
            display("You have'nt created dictionaries", Color::Red);
            display("ex) wordic add sample value", Color::Red);
        }
    }

    pub fn register(&self, dictionary: Dictionary) -> Result<(), String> {
        self.dictionary_repository.register(dictionary)?;
        display("Created new dictionary!", Color::Green);
        Ok(())
    }

    pub fn save(&self, dictionaries: Vec<Dictionary>) -> Result<(), String> {
        self.dictionary_repository.save(dictionaries)
    }

    pub fn get(&self, key: String) -> Result<String, String> {
        self.dictionary_repository.get(key)
    }

    pub fn remove(&self, key: String) -> Vec<Dictionary> {
        self.dictionary_repository.remove(key)
    }
}
