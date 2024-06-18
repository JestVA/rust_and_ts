#![allow(dead_code)]

enum Color {
    Red,
    Green,
    Blue
}

fn print_color(color: Color) {
    match color {
        Color::Red => println!("red"),
        Color::Green => println!("green"),
        Color::Blue => println!("blue"),
    }
}
fn main() {
    print_color(Color::Green)
}