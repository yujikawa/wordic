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
    fn get_dict(&self, key: String) -> Option<Dictionary>;
    fn remove(&self, key: String) -> Vec<Dictionary>;
    fn search(&self, keyword: String) -> Vec<Dictionary>;
    fn export(&self, path: &str) -> Result<(), String>;
    fn import(&self, path: &str) -> Result<(), String>;
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
            if let Err(why) = fs::create_dir(save_dir) {
                println!("! {:?}", why.kind())
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
        file.write_all(serialized.as_bytes()).unwrap();
        Ok(())
    }

    fn get(&self, key: String) -> Result<String, String> {
        Ok(self.get_dict(key).map(|d| d.value).unwrap_or_default())
    }

    fn get_dict(&self, key: String) -> Option<Dictionary> {
        self.load_data().into_iter().find(|d| d.key == key)
    }

    fn remove(&self, key: String) -> Vec<Dictionary> {
        let mut dictionaries: Vec<Dictionary> = self.load_data();
        dictionaries.retain(|x| x.key != key);
        dictionaries
    }

    fn search(&self, keyword: String) -> Vec<Dictionary> {
        self.load_data()
            .into_iter()
            .filter(|d| {
                d.key.contains(&keyword)
                    || d.value.contains(&keyword)
                    || d.description.contains(&keyword)
            })
            .collect()
    }

    fn export(&self, path: &str) -> Result<(), String> {
        let dictionaries = self.load_data();
        let serialized = serde_json::to_string(&dictionaries).map_err(|e| e.to_string())?;
        let mut file = fs::File::create(path).map_err(|e| e.to_string())?;
        file.write_all(serialized.as_bytes())
            .map_err(|e| e.to_string())
    }

    fn import(&self, path: &str) -> Result<(), String> {
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        let imported: Vec<Dictionary> =
            serde_json::from_str(&content).map_err(|e| e.to_string())?;
        self.save(imported)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::Mutex;

    // All tests share ~/.wordic/wordic.json, so run them serially.
    static LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn test_repository() {
        let _guard = LOCK.lock().unwrap();
        let dictionary_repository = DictionaryJsonRepository::new();
        let _ = dictionary_repository.init().unwrap();
        let value = dictionary_repository
            .register(Dictionary::new(
                String::from("sample"),
                String::from("sample_value"),
                String::from(""),
            ))
            .unwrap();
        assert!(() == value);
        let value = dictionary_repository.get(String::from("sample")).unwrap();
        assert!(value == "sample_value");
    }

    #[test]
    fn test_get_dict() {
        let _guard = LOCK.lock().unwrap();
        let repo = DictionaryJsonRepository::new();
        let _ = repo.init();
        let _ = repo.register(Dictionary::new(
            String::from("get_dict_key"),
            String::from("get_dict_value"),
            String::from("get_dict_desc"),
        ));
        let result = repo.get_dict(String::from("get_dict_key"));
        assert!(result.is_some());
        let dict = result.unwrap();
        assert_eq!(dict.value, "get_dict_value");
        assert_eq!(dict.description, "get_dict_desc");

        let none = repo.get_dict(String::from("zzz_not_exist_zzz"));
        assert!(none.is_none());
    }

    #[test]
    fn test_search() {
        let _guard = LOCK.lock().unwrap();
        let repo = DictionaryJsonRepository::new();
        let _ = repo.init();
        let _ = repo.register(Dictionary::new(
            String::from("search_key_rust"),
            String::from("systems programming language"),
            String::from("search_desc"),
        ));
        let _ = repo.register(Dictionary::new(
            String::from("search_key_python"),
            String::from("scripting language"),
            String::from("search_desc"),
        ));

        // key match
        let results = repo.search(String::from("search_key_rust"));
        assert!(results.iter().any(|d| d.key == "search_key_rust"));
        assert!(!results.iter().any(|d| d.key == "search_key_python"));

        // value match
        let results = repo.search(String::from("systems programming"));
        assert!(results.iter().any(|d| d.key == "search_key_rust"));

        // description match
        let results = repo.search(String::from("search_desc"));
        assert!(results.iter().any(|d| d.key == "search_key_rust"));
        assert!(results.iter().any(|d| d.key == "search_key_python"));

        // no match
        let results = repo.search(String::from("zzz_no_match_zzz"));
        assert!(!results.iter().any(|d| d.key == "search_key_rust"));
    }

    #[test]
    fn test_export_import() {
        let _guard = LOCK.lock().unwrap();
        let repo = DictionaryJsonRepository::new();
        let _ = repo.init();
        let _ = repo.register(Dictionary::new(
            String::from("export_test_key"),
            String::from("export_test_value"),
            String::from("export_test_desc"),
        ));

        let export_path = "/tmp/wordic_test_export.json";
        assert!(repo.export(export_path).is_ok());
        assert!(std::path::Path::new(export_path).exists());

        // wipe data, then import
        let _ = repo.save(vec![]);
        assert_eq!(repo.get(String::from("export_test_key")).unwrap(), "");

        assert!(repo.import(export_path).is_ok());
        assert_eq!(
            repo.get(String::from("export_test_key")).unwrap(),
            "export_test_value"
        );
    }
}
