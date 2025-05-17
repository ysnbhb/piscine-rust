use pangram::*;

fn main() {
    println!(
        "{}",
        is_pangram("")
    );
    println!("{}", is_pangram("this is not a pangram!"));
}