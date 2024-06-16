fn main() {
    let file = std::fs::read_to_string("project/lines").unwrap();
 
    file.lines()
        .enumerate() // every single index
        .filter(|(idx, _)| idx % 2 == 0)
        .for_each(|(_, line)| println!("{}", line));
}