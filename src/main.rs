use std::io::{stdin};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Unable to read input");

    let mut result = 0;
    for slice in input.split_whitespace() {
        result += slice.parse::<i32>().expect("Unable to parse given value");
    }

    println!("{result}");
}
