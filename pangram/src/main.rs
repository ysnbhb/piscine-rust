use pangram::*;

fn main() {
    println!(
        "{}",
        is_pangram("Victor jagt zwölf Boxkämpfer quer über den großen Sylter Deich")
    );
    // println!("{}", is_pangram("this is not a pangram!"));
}