use clap::{Parser, Subcommand};
use std::io;

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

struct Wordic {
    key: String,
    value: String,
}

impl Wordic {
    fn new(key: &str, value: &str) -> Self {
        Wordic {
            key: key.to_string(),
            value: value.to_string(),
        }
    }
}

/// Register data
fn register(wd: Wordic) -> Result<(), String> {
    todo!();
}

fn get(key: &str) -> Result<String, String> {
    todo!();
}

fn find(key: &str) -> Result<bool, String> {
    todo!();
}

fn search(key: &str) -> Vec<Wordic> {
    vec![]
}

fn main() {
    let cli = Cli::parse();
    dbg!(cli.subcommand);
    // println!("Hello {}!{}", cli.subcommand);
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup() {
        register(Wordic::new("sample1", "sample_value1"));
        register(Wordic::new("sample2", "sample_value2"));
        register(Wordic::new("sample3", "sample_value3"));
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
        assert!(value == "value_sample1".to_string());
    }

    #[test]
    fn test_find() {
        setup();
        let value = find("sample1").unwrap();
        assert!(value);
    }

    #[test]
    fn test_search() {
        setup();
        let value = search("sample");
        assert!(value.len() == 3);
    }
}