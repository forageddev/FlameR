fn main() {
    println!("---------------------------------------------");
    println!("Flame recreation for Rust made by Foraged at 11 pm.");
    println!("---------------------------------------------");
    let mut firstName = get_input("Enter the first name.").to_lowercase();
    let mut secondName = get_input("Enter the second name.").to_lowercase();

    for c in firstName.clone().chars() {
        if firstName.contains(c) && secondName.contains(c) {
            firstName = firstName.replace(c, "");
            secondName = secondName.replace(c, "");
        }
    }

    let mut chars = vec!["F", "L", "A", "M", "E"];
    let mut charAmount = firstName.len() + secondName.len();

  
    for z in 0..4 {
        let mut f = 0;

        for i in 0..(charAmount - 1) {
            f = f + 1;
            if f == chars.iter().count() {
                f = 0;
            }
        }

        chars.remove(f);
    }

    println!("---------------------------------------------");
    println!("The result is {}", format(chars[0]));
    println!("---------------------------------------------");
}

use std::io;
pub fn get_input(prompt: &str) -> String{
    println!("{}", prompt);
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {},
        Err(_no_updates_is_fine) => {},
    }
    input.trim().to_string()
}

pub fn format(result: &str) -> &str {
    if result == "F" {
        return "Friend";
    }
    if result == "L" {
        return "Love";
    }
    if result == "A" {
        return "Affection";
    }
    if result == "M" {
        return "Marry";
    }
    if result == "E" {
        return "Enemy";
    }

    return "Error";
}