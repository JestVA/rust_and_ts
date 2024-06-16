fn main() {
    let file = std::fs::read_to_string("project/lines").unwrap();
 
    file.lines().for_each(|line| println!("{}", line));
}