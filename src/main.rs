#![allow(dead_code)]
struct Custom {
    age: usize,
    name: String
}

// doing this at a language level! Great (no type guards required like in Ts)
enum Item {
    Number(usize),
    String(String),
    MyCustom(Custom),
}

fn append(items: &mut Vec<Item>) {
    items.push(Item::String("Hello again".to_string())); // make a string here <---
}

fn main() {
    let mut items : Vec<Item> = vec![];
    append(&mut items);   

    // calling some or none
    let _foo = Some(5);
    let _foo : Option<String> = None;

}
