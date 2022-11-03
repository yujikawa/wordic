use clap::{Parser, Subcommand};
use dirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    subcommand: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    List,

    #[clap(arg_required_else_help = true)]
    Add {
        key: String,
        value: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Wordic {
    key: String,
    value: String,
    description: String,
}

impl Wordic {
    fn new(key: &str, value: &str) -> Self {
        Wordic {
            key: key.to_string(),
            value: value.to_string(),
            description: "".to_string(),
        }
    }
}

fn init() {
    let save_dir = get_home_path();
    if !save_dir.exists() {
        match fs::create_dir(save_dir) {
            Err(why) => println!("! {:?}", why.kind()),
            Ok(_) => {}
        }
    }
    if !get_dict_path().exists() {
        save(vec![]).unwrap();
    }
}

fn get_home_path() -> PathBuf {
    let home_dir = dirs::home_dir().unwrap();
    Path::new(&home_dir).join(".wordic")
}

fn get_dict_path() -> PathBuf {
    get_home_path().join("wordic.json")
}

/// Register data
fn register(wd: Wordic) -> Result<(), String> {
    let mut wds = remove(wd.key.as_str());
    wds.push(wd);
    save(wds).unwrap();
    Ok(())
}

fn load_data() -> Vec<Wordic> {
    let file_path = get_dict_path();
    let content = fs::read_to_string(file_path).expect("Failed to load JSON");
    let deserialized: Vec<Wordic> = serde_json::from_str(&content).unwrap();
    deserialized
}

fn save(wds: Vec<Wordic>) -> Result<(), String> {
    let file_path = get_dict_path();
    let mut serialized = serde_json::to_string(&wds).unwrap();
    let mut file = fs::File::create(file_path).unwrap();
    file.write_all(serialized.as_bytes());
    Ok(())
}

fn get(key: &str) -> Result<String, String> {
    let mut value: String = "".to_string();
    let wds = load_data();
    for i in wds {
        if i.key.as_str() == key {
            value = i.value;
        }
    }
    Ok(value)
}

fn show() {
    let wds = load_data();
    for (i, v) in wds.iter().enumerate() {
        println!("{} : {}", i, v.key);
    }
}

fn remove(key: &str) -> Vec<Wordic> {
    let mut wds: Vec<Wordic> = load_data();
    wds.retain(|x| x.key != key);
    wds
}

fn main() {
    // let cli = Cli::parse();
    // dbg!(cli.subcommand);
    // println!("Hello {}!{}", cli.subcommand);
    init();
    register(Wordic::new("sample1", "sample_value1")).unwrap();
    register(Wordic::new("sample2", "sample_value2")).unwrap();
    register(Wordic::new("sample3", "sample_value3")).unwrap();
    show();
    println!("value={}", get("sample1").unwrap());
    // register(Wordic::new("sample2", "sample_value2"));
    // register(Wordic::new("sample3", "sample_value3"));
    // remove("sample2");
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup() {
        register(Wordic::new("sample1", "sample_value1")).unwrap();
        register(Wordic::new("sample2", "sample_value2")).unwrap();
        register(Wordic::new("sample3", "sample_value3")).unwrap();
    }

    #[test]
    fn test_register() {
        let value = register(Wordic::new("sample", "sample_value")).unwrap();
        assert!(() == value);
    }

    #[test]
    fn test_get() {
        setup();
        let value = get("sample1").unwrap();
        assert!(value == "sample_value1");
    }
}
