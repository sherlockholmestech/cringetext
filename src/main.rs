use std::env;
use rand;
fn main() {
    let args: Vec<String> = env::args().collect();
    let text = args.get(1).expect("ERR 01 -- Unable to get argument");
    let mut cringetext = String::new();
    let mut cap_streak = 0;
    let mut capital: bool = rand::random::<bool>();
    if text == "-h" || text == "--help" {
        cringetext = String::from("CringeText.\nCopyright (c) 2022 Sherlock Holmes\n\nUsage:\ncringetext [TEXT]");
    } else {
        for char in text.chars() {
            if capital == false {
                cringetext.push(char.to_ascii_lowercase());
                capital = rand::random();
            } else {
                cringetext.push(char.to_ascii_uppercase());
                capital = rand::random();
                cap_streak += 1;
                if cap_streak == 2 {
                    capital = false;
                    cap_streak = 0;
                }
            }
        }
    }
    println!("{}", cringetext);
}
