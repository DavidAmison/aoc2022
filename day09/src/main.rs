use utils::{parse_field_unwrap, files};
use std::collections::HashMap;

fn main() {
    let input = files::read_in_lines("input").iter()
        .map(|l| parse_field_unwrap!(l => char, " " | isize, ""))
        .collect::<Vec<(char, isize)>>();

    // let mut map = [['.'; 10]; 10];

    println!("\n----- PART 1 -----\n");

    let mut head: (isize, isize) = (9, 0);
    let mut tail: (isize, isize) = (9, 0);
    let mut visited = HashMap::new();
    let mut last_head: char = '.';

    for (i, d) in input.iter() {
        for _ in 0..*d {
            // map[tail.0 as usize][tail.1 as usize] = '#';
            // map[head.0 as usize][head.1 as usize] = last_head;
            update_head_position(&mut head, *i);
            update_tail_position(&mut tail, &head);
            let t = tail.clone();
            let p = visited.entry(t).or_insert(0);
            *p += 1;
            // map[tail.0 as usize][tail.1 as usize] = 'T';
            // last_head = map[head.0 as usize][head.1 as usize];
            // map[head.0 as usize][head.1 as usize] = 'H';

            // for row in map.iter() {
            //     for c in row.iter() {
            //         print!("{}", c);
            //     }
            //     println!("");
            // }
            // println!("");
            // println!("");
        }
    }

    let answer1 = visited.iter().count();

    println!("Part 1 answer: {}", answer1);

    println!("\n----- PART 2 -----\n");

    const n: usize = 10; // Num of knots
    let mut knots = [(0isize, 0isize); n];

    visited.clear();

    for (i, d) in input.iter() {
        for _ in 0..*d {
            update_head_position(&mut knots[0], *i);
            for i in 1..n {
                let prev_knot = knots[i-1].clone();
                update_tail_position(&mut knots[i], &prev_knot);
            }
            // Get position of tail
            let t = knots[n-1].clone();
            let p = visited.entry(t).or_insert(0);
            *p += 1;
        }
    }



    let answer2 = visited.iter().count();

    println!("Part 2 answer: {}", answer2);
}

fn update_tail_position(t: &mut (isize, isize), h: &(isize, isize)) {
    // println!("HEAD IS {},{}, TAIL IS {},{}", h.0, h.1, t.0, t.1);
    let d = (h.0 - t.0, h.1 - t.1);
    // println!("DELTA IS {},{}", d.0, d.1);
    if d.0.abs() == 2 || d.1.abs() == 2 {
        t.0 += if d.0 > 0 { 1 } else if d.0 < 0 { -1 } else { 0 };
        t.1 += if d.1 > 0 { 1 } else if d.1 < 0 { -1 } else { 0 };
    }
}

fn update_head_position(h: &mut (isize, isize), d: char) {
    match d {
        'U' => h.0 -= 1,
        'D' => h.0 += 1,
        'L' => h.1 -= 1,
        'R' => h.1 += 1,
        _ => panic!("UNRECOGNISED INSTRUCTION"),
    };
}
