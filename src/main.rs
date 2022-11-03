use clap::{Parser, Subcommand};

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
    Add {key: String, value: String,},
}

// fn get_list() {
//  todo!()
// }

// fn add_value(_key: String, _value: String) {
//     todo!()
// }

fn main() {
    let cli = Cli::parse();
    dbg!(cli.subcommand);
    // println!("Hello {}!{}", cli.subcommand);
}
