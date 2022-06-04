use clap::Command;

fn main() {
    let _matches = Command::new("wordic")
        .version("0.1.0")
        .author("Yujikawa <ykyujikawa@gmail.com>")
        .about("wordic is dictoinary")
        .get_matches();
}
