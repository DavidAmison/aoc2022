use utils::{parse_field_unwrap, files};

fn main() {

    // [M]                     [N] [Z]
    // [F]             [R] [Z] [C] [C]
    // [C]     [V]     [L] [N] [G] [V]
    // [W]     [L]     [T] [H] [V] [F] [H]
    // [T]     [T] [W] [F] [B] [P] [J] [L]
    // [D] [L] [H] [J] [C] [G] [S] [R] [M]
    // [L] [B] [C] [P] [S] [D] [M] [Q] [P]
    // [B] [N] [J] [S] [Z] [W] [F] [W] [R]
    //  1   2   3   4   5   6   7   8   9

    let mut s = vec!(
        vec!('B', 'L', 'D', 'T', 'W', 'C', 'F', 'M'),
        vec!('N', 'B', 'L'),
        vec!('J', 'C', 'H', 'T', 'L', 'V'),
        vec!('S', 'P', 'J', 'W'),
        vec!('Z', 'S', 'C', 'F', 'T', 'L', 'R'),
        vec!('W', 'D', 'G', 'B', 'H', 'N', 'Z'),
        vec!('F', 'M', 'S', 'P', 'V', 'G', 'C', 'N'),
        vec!('W', 'Q', 'R', 'J', 'F', 'V', 'C', 'Z'),
        vec!('R', 'P', 'M', 'L', 'H')
    );

    let mut s2 = vec!(
        vec!('B', 'L', 'D', 'T', 'W', 'C', 'F', 'M'),
        vec!('N', 'B', 'L'),
        vec!('J', 'C', 'H', 'T', 'L', 'V'),
        vec!('S', 'P', 'J', 'W'),
        vec!('Z', 'S', 'C', 'F', 'T', 'L', 'R'),
        vec!('W', 'D', 'G', 'B', 'H', 'N', 'Z'),
        vec!('F', 'M', 'S', 'P', 'V', 'G', 'C', 'N'),
        vec!('W', 'Q', 'R', 'J', 'F', 'V', 'C', 'Z'),
        vec!('R', 'P', 'M', 'L', 'H')
    );

    let instr = files::read_in_csv_matrix_as::<usize>("input");


    println!("\n----- PART 1 -----\n");

    for i in instr.iter() {
        for _ in 0..i[0] {
            let e = s[i[1] - 1].pop().unwrap();
            s[i[2] - 1].push(e);
        }
    }

    print!("Part 1 answer: ");
    for c in s.iter() {
        print!("{}", c.last().unwrap());
    }
    println!();

    println!("\n----- PART 2 -----\n");

    for i in instr.iter() {
        let n = s2[i[1] - 1].len() - i[0];
        let mut e: Vec<char> = s2[i[1] - 1].drain(n..).collect();
        s2[i[2] - 1].append(&mut e);
    }

    let answer1 = 0;

    let answer2 = 0;

    print!("Part 2 answer: ");
    for c in s2.iter() {
        print!("{}", c.last().unwrap());
    }
    println!();
}
