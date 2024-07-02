//#![allow(dead_code)]
//struct Custom {
//    age: usize,
//    name: String
//}

//// doing this at a language level! Great (no type guards required like in Ts)
//enum Item {
//    Number(usize),
//    String(String),
//    MyCustom(Custom),
//}

//fn append(items: &mut Vec<Item>) {
//    items.push(Item::String("Hello again".to_string())); // make a string here <---
//}

//fn main() {
//    let mut items : Vec<Item> = vec![];
//    append(&mut items);   

//    // calling some or none
//    let _foo = Some(5);
//    let _foo : Option<String> = None;

//}



// Exploring traits and interfaces
#![allow(dead_code)]

use std::f64::consts::PI;

struct Rect {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

struct Circle {
    x: f64,
    y: f64,
    radius: f64
}

trait Area {
    fn area(&self) -> f64;
}

impl Area for Rect {
    fn area(&self) -> f64 {
        return self.width * self.height
    }
}

impl Area for Circle {
    fn area(&self) -> f64 {
        return self.radius * self.radius * PI
    }
}

fn main() {
    let rect = Rect {
        x: 0.0,
        y: 0.0,
        width: 10.0,
        height: 10.0
    };

    let circ = Circle {
        x: 0.0,
        y: 0.0,
        radius: 10.0
    };

    println!("{}", circ.area());
    println!("{}", rect.area());
}