use super::super::domain::dictionary::Dictionary;
use dirs;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

pub trait IDictionaryRepository {
    fn new() -> Self;
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

pub struct DictionaryJsonRepository {
    file_name: String,
}
impl IDictionaryRepository for DictionaryJsonRepository {
    fn new() -> DictionaryJsonRepository {
        DictionaryJsonRepository {
            file_name: "wordic.json".to_string(),
        }
    }
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
        let _value = dictionary_repository
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
