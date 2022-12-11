use utils::{parse_field_unwrap, files};

// Fully contains
fn part1(r: (u32, u32, u32, u32)) -> bool {
    (r.0 >= r.2 && r.1 <= r.3) || (r.2 >= r.0 && r.3 <= r.1)
}

// Partially contains
fn part2(r: (u32, u32, u32, u32)) -> bool {
    (r.0 >= r.2 && r.0 <= r.3) || (r.1 >= r.2 && r.1 <= r.3) ||
    (r.2 >= r.0 && r.2 <= r.1) || (r.3 >= r.0 && r.3 <= r.1)
}

fn main() {
    let values = files::read_in_lines("input")
        .iter()
        .map(|l| parse_field_unwrap!(l => u32 , "-" | u32 , "," | u32 , "-" | u32 , ""))
        .collect::<Vec<(u32, u32, u32, u32)>>();

    println!("\n----- PART 1 -----\n");

    let answer1 = values.iter()
        .filter(|r| part1(**r))
        .count();

    println!("Part 1 answer: {}", answer1);

    println!("\n----- PART 2 -----\n");

    let answer2 = values.iter()
    .filter(|r| part2(**r))
    .count();

    println!("Part 2 answer: {}", answer2);
}
