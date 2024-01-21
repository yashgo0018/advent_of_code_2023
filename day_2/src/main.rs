use std::fs;

fn solve() -> u32{
    let content = fs::read_to_string("./input.txt").expect("The file should be there").trim().to_string();

    let lines = content.split("\n");
    let mut sum_of_ids = 0;
    for line in lines {
        let parts: Vec<&str> = line.split(": ").collect();
        let game_id: u32 = (&parts[0][5..]).parse().expect("it should be a number");
        let mut is_valid = true;
        for game_round in parts[1].split("; ") {
            for cube_count in game_round.split(", ") {
                let parts1: Vec<&str> = cube_count.split(" ").collect();
                let count: u32 = parts1[0].parse().expect("it should be number");
                let cube_type = parts1[1];
                is_valid &= match cube_type {
                    "red" => count <= 12,
                    "green" => count <= 13,
                    "blue" => count <= 14,
                    _ => true
                }
            }
        }
        if is_valid {
            sum_of_ids += game_id;
        }
    }
    sum_of_ids
}


fn main() {
    println!("{}", solve());
}
