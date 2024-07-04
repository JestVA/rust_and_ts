use anyhow::Result;

fn main() -> Result<()> {
   // println!("Hello world of Rustz...!"); 
   // Howdy boyz and girlz. Using anyhow to return a better Result type which can be reasonably
   // used in return types for main functions and in general throughout Rust code

    for i in 1..=100 {
        match i {
            n if n % 3 == 0 && n % 5 == 0 => println!("FizzBuzz"),
            n if n % 3 == 0 => println!("Fizzbuzz"),
            n if n % 5 == 0 => println!("BUzz"),
            n => println!("{n}"),
        }
    }

    Ok(())
}
