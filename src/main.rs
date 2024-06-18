#![allow(dead_code)]

enum Color {
    Red,
    Green,
    Blue,
    Yellow,
}

impl Color {
    fn is_green(&self) -> bool {
        if let Color::Green = self {
            return true;
        }
        return false
    }

    fn is_green_parts(&self) -> bool {
        match self {
            Color::Red => return false,
            Color::Green => return false,
            Color::Blue => return true,
            Color::Yellow => return true,
        }
    }
}

fn print_color(color: Color) {
    match color {
        Color::Red => println!("red"),
        Color::Green => println!("green"),
        Color::Blue => println!("blue"),
        Color::Yellow => println!("yellow")
    }
}
fn main() {
    print_color(Color::Green);

    let foo = Color::Green;

    let is_green : bool = foo.is_green();
    println!("IS GREEN {is_green}");

    let is_green_parts : bool = foo.is_green_parts();
    println!("IS GREEN PARTS {is_green_parts}");
}