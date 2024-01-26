use std::cmp::{max, min};
use std::fs;

fn check_if_symbol_exists(line: &str, s: i32, e: i32) -> bool {
    for i in max(0, s) as usize..min(line.len() as i32,e + 1) as usize {
        let c = line.chars().nth(i).unwrap();
        if !c.is_digit(10) && c != '.' {
            return true;
        }
    }

    false
}

fn check_if_number_is_required(lines: &Vec<&str>, l: usize, s: i32, e: i32) -> u32 {
    let line = lines[l];
    let number_string = &line[s as usize..(e+1) as usize];
    let number: u32 = number_string.parse().unwrap();

    if check_if_symbol_exists(line, s - 1, e + 1) {
        return number;
    }

    if l != 0 {
        let prev_line = lines[l - 1];
        if check_if_symbol_exists(prev_line, s - 1, e + 1) {
            return number;
        }
    }

    if l + 1 != lines.len() {
        let next_line = lines[l + 1];
        if check_if_symbol_exists(next_line, s - 1, e + 1) {
            return number;
        }
    }

    0
}

fn main() {
    let content = fs::read_to_string("./input.txt").expect("The file should be there").trim().to_string();
    let lines: Vec<&str> = content.split("\n").collect();

    let mut total = 0;

    for i in 0..lines.len() {
        let line = lines[i];
        let mut start: i32 = -1;
        for j in 0..line.len() {
            let c = line.chars().nth(j).unwrap();

            if c.is_digit(10) {
                if start == -1 {
                    start = j as i32;
                }

                if j == line.len() - 1 {
                    total += check_if_number_is_required(&lines, i, start, j as i32);
                }
            } else if start != -1 {
                total += check_if_number_is_required(&lines, i, start, (j - 1) as i32);
                start = -1;
            }
        }
    }
    println!("{}", total);
}
