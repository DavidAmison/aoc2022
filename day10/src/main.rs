use utils::{parse_field, files};

fn main() {
    let input: Vec<(String, Option<i32>)> = files::read_in_lines("input").iter()
        .map(|l| parse_field!(l => String, " "| i32, ""))
        .map(|(i, v)| (i.unwrap(), v))
        .collect();

    println!("\n----- PART 1 -----\n");

    let mut x = 1;
    let mut cycle = 1;
    let mut answer1 = 0;

    for (i, v) in input.iter() {
        match (i.as_str(), v) {
            ("noop", _) => {
                answer1 += check_cycle(x, cycle);
                cycle += 1;
            }
            ("addx", Some(v)) => {
                answer1 += check_cycle(x, cycle);
                cycle += 1;
                answer1 += check_cycle(x, cycle);
                cycle += 1;
                x += v;
            }
            _ => ()
        }
    }


    println!("Part 1 answer: {}", answer1);

    println!("\n----- PART 2 -----\n");

    x = 1;
    cycle = 1;
    let mut screen = [[false; 40]; 6];

    for (i, v) in input.iter() {
        match (i.as_str(), v) {
            ("noop", _) => {
                draw_screen(x, cycle, &mut screen);
                cycle += 1;
            }
            ("addx", Some(v)) => {
                draw_screen(x, cycle, &mut screen);
                cycle += 1;
                draw_screen(x, cycle, &mut screen);
                cycle += 1;
                x += v;
            }
            _ => ()
        }
    }

    for row in screen {
        for s in row {
            print!("{}", if s {'#' } else { ' ' })
        }
        println!();
    }
    let answer2 = 0;

    println!("Part 2 answer: {}", answer2);
}

fn check_cycle(x: i32, cycle: usize) -> i32 {
    if cycle%40 == 20 {
        println!("x at cycle {} is {} -> {}", cycle, x, (cycle as i32)*x);
        (cycle as i32)*x
    } else {
        0
    }
}

fn draw_screen(x: i32, cycle: usize, screen: &mut [[bool; 40]; 6]) {
    println!("cycle {} - x {}", cycle, x);
    let row = (cycle - 1) / 40;
    let pixel = (cycle - 1) % 40;
    if ((pixel as i32) - x).abs() <= 1 {
        println!("MATCH AT CYCLE {} ({})", cycle, x);
        screen[row][pixel] = true;
    }
}
