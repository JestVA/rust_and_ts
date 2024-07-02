use crate::shapes::{area::Area, circle::Circle, rect::Rect};

/*
Exploring traits and interfaces
*/
mod shapes;

fn main() {
    let rect = Rect {
        x: 0.0,
        y: 0.0,
        width: 10.0,
        height: 10.0,
    };

    let circ = Circle {
        x: 0.0,
        y: 0.0,
        radius: 10.0,
    };

    println!("{}", circ.area());
    println!("{}", rect.area());
}
