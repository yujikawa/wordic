use super::super::domain::dictionary::Dictionary;
use super::super::functions::display::{display, Color};
use super::super::infrastructure::repository::IDictionaryRepository;

pub struct DictionaryApplicationService<T> {
    dictionary_repository: T,
}

impl<T: IDictionaryRepository> DictionaryApplicationService<T> {
    pub fn new(dictionary_repository: T) -> Self {
        DictionaryApplicationService {
            dictionary_repository,
        }
    }

    pub fn init(&self) -> Result<(), String> {
        self.dictionary_repository.init()
    }

    pub fn show(&self) {
        let dictionaries = self.dictionary_repository.load_data();
        let len = dictionaries.len();
        if len > 0 {
            display(
                &format!("You have {} word(s) in your dictionary", len),
                Color::Green,
            );
            println!();
            println!(
                "{0: <5} | {1: <20} | {2: <25} | {3: <25} | {4: <30}",
                "No", "Key", "Value", "Description", "Created at"
            );
            println!(
                "{0:-<5}-+-{1:-<20}-+-{2:-<25}-+-{3:-<25}-+-{4:-<30}",
                "", "", "", "", ""
            );
            for (i, v) in dictionaries.iter().enumerate() {
                let value = truncate(&v.value, 25);
                let description = truncate(&v.description, 25);
                println!(
                    "{0: <5} | {1: <20} | {2: <25} | {3: <25} | {4: <30}",
                    i, v.key, value, description, v.timestamp
                );
            }
        } else {
            display("Your dictionary is empty.", Color::Red);
            display("ex) wordic add <key> <value>", Color::Red);
        }
    }

    pub fn register(&self, dictionary: Dictionary) -> Result<(), String> {
        self.dictionary_repository.register(dictionary)?;
        display("Created new dictionary!", Color::Green);
        Ok(())
    }

    pub fn get(&self, key: String) -> Result<String, String> {
        self.dictionary_repository.get(key)
    }

    pub fn get_full(&self, key: String) {
        match self.dictionary_repository.get_dict(key) {
            Some(d) => {
                println!("Key        : {}", d.key);
                println!("Value      : {}", d.value);
                println!("Description: {}", d.description);
                println!("Created at : {}", d.timestamp);
            }
            None => display("Not found in your dictionary.", Color::Red),
        }
    }

    pub fn search(&self, keyword: String) {
        let results = self.dictionary_repository.search(keyword.clone());
        if results.is_empty() {
            display(&format!("No results for \"{}\".", keyword), Color::Red);
        } else {
            display(&format!("{} result(s) found.", results.len()), Color::Green);
            println!();
            println!(
                "{0: <5} | {1: <20} | {2: <25} | {3: <25} | {4: <30}",
                "No", "Key", "Value", "Description", "Created at"
            );
            println!(
                "{0:-<5}-+-{1:-<20}-+-{2:-<25}-+-{3:-<25}-+-{4:-<30}",
                "", "", "", "", ""
            );
            for (i, v) in results.iter().enumerate() {
                let value = truncate(&v.value, 25);
                let description = truncate(&v.description, 25);
                println!(
                    "{0: <5} | {1: <20} | {2: <25} | {3: <25} | {4: <30}",
                    i, v.key, value, description, v.timestamp
                );
            }
        }
    }

    pub fn save(&self, dictionaries: Vec<Dictionary>) -> Result<(), String> {
        self.dictionary_repository.save(dictionaries)
    }

    pub fn remove(&self, key: String) -> Vec<Dictionary> {
        self.dictionary_repository.remove(key)
    }

    pub fn export(&self, path: String) -> Result<(), String> {
        self.dictionary_repository.export(&path)?;
        display(&format!("Exported to {}", path), Color::Green);
        Ok(())
    }

    pub fn import(&self, path: String) -> Result<(), String> {
        self.dictionary_repository.import(&path)?;
        display(&format!("Imported from {}", path), Color::Green);
        self.show();
        Ok(())
    }
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() > max {
        format!("{}..", &s[..max - 2])
    } else {
        s.to_string()
    }
}
