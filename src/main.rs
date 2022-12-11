use std::env;
use rand;
fn main() {
    let args: Vec<String> = env::args().collect();
    let text = args.get(1).expect("ERR 01 -- Unable to get argument");
    let mut cringetext = String::new();
    let mut capital: bool = rand::random::<bool>();
    for char in text.chars() {
        if capital == false {
            cringetext.push(char.to_ascii_lowercase());
            capital = rand::random();
        } else {
            cringetext.push(char.to_ascii_uppercase());
            capital = rand::random();
        }
    }
    println!("{}", cringetext);
}
