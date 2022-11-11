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

            for (i, v) in dictionaries.iter().enumerate() {
                if v.description.len() == 0 {
                    println!("{} : {}", i, v.key);
                } else {
                    println!("{} : {} <{}> ", i, v.key, v.description);
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
