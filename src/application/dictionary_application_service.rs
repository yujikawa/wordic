use super::super::domain::dictionary::Dictionary;
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
        for (i, v) in dictionaries.iter().enumerate() {
            if v.description.len() == 0 {
                println!("{} : {}", i, v.key);
            } else {
                println!("{} : {} <{}> ", i, v.key, v.description);
            }
        }
    }

    pub fn register(&self, dictionary: Dictionary) -> Result<(), String> {
        self.dictionary_repository.register(dictionary)
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
