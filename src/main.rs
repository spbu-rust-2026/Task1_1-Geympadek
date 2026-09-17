use std::io::stdin;

type Int = i128;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Unable to read input");

    let mut result: Int = 0;
    for slice in input.split_whitespace() {
        result += slice.parse::<Int>().expect("Unable to parse given value");
    }

    print!("{result}");
}
