pub enum Color {
    Green,
    Red,
}

pub fn display(text: &str, color: Color) {
    match color {
        Color::Green => println!("\x1b[{}m\x1b[47m{}\x1b[m ", 32, text),
        Color::Red => println!("\x1b[{}m\x1b[47m{}\x1b[m ", 31, text),
    }
}
