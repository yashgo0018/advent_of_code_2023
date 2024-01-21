use std::fs;

fn solve() -> u32 {
    let content = fs::read_to_string("./input.txt").expect("The file should exists");

    let lines = content.split("\n");
    let mut sum = 0;
    for line in lines {
        let mut number = 0;
        let mut first_found = false;
        for c in line.chars() {
            if c.is_digit(10) {
                let digit = c.to_digit(10).unwrap();
                if first_found {
                    number = (number / 10) * 10 + digit;
                } else {
                    number = digit * 11;
                    first_found = true;
                }
            }
        }
        sum += number;
    }
    sum
}

fn main() {
    println!("{}", solve());
}
