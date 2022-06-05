use clap::{Command, Arg};
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    name: String,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}
fn main() {
    let key = Arg::new("key")
    .help("Input key")
    .short('k')
    .long("key")
    .takes_value(true)
    .required(true);

    let value = Arg::new("value")
    .help("Input value")
    .short('v')
    .long("value")
    .takes_value(true)
    .required(true);

    let secret_op = Arg::new("secret")
    .help("Mask value")
    .short('s')
    .long("secret")
    .required(false);
    // subcommands
    let add_cmd = Command::new("add")
    .about("Add new dictionary")
    .arg(key)
    .arg(value)
    .arg(secret_op);

    let list_cmd = Command::new("list")
    .about("Get dictionaries");



    let _matches = Command::new("wordic")
        .version("0.1.0")
        .author("Yujikawa <ykyujikawa@gmail.com>")
        .about("wordic is dictoinary")
        .subcommand(add_cmd)
        .subcommand(list_cmd)
        .get_matches();
}
