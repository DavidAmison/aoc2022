use utils::{parse_field_unwrap, files};
use std::collections::HashSet;

fn main() {
    // Vectors of co-ordinates (row, column)
    // Row increases upward - column increases right
    let rocks = vec!(vec!((0,0), (0,1), (0,2), (0,3)),
        vec!((0,1), (1,0), (1,1), (1,2), (2,1)),
        vec!((0,0), (0,1), (0,2), (1,2), (2,2)),
        vec!((0,0), (1,0), (2,0), (3,0)),
        vec!((0,0), (0,1), (1,0), (1,1)));


    let values = files::read_in_line("input");

    println!("\n----- PART 1 -----\n");

    // Floow is at 0
    let floor = 0;
    // Wall at 0 and 8 (empty space is 7 wide)
    let left_wall = 0;
    let right_wall = 8;
    // Number of rocks to fall
    let mut max_count = 100000;

    let mut count = 0;
    let mut rock_n = 0;
    let mut rock_stopped = false;
    let mut new_rock = true;
    let mut highest_point = 0;
    let mut points: HashSet<(usize, usize)> = HashSet::new();
    let mut rock = vec!();
    while count < max_count {
        println!("COUNT = {}", count);
        println!("REPEATING AT rock_n {}, new_rock {}", rock_n, new_rock);
        println!("ROCK AT {:?}", rock);
        println!("HEIGHT IS {:?}", highest_point);
        if count > 50000 {
            max_count = count + 132;
        }
        for c in values.chars() {
            if new_rock {
                // println!("NEXT ROCK");
                // Find the 'starting' position of the rock
                // Bottom of each rack will always be 0 - left will always be 0
                rock = rocks[rock_n].clone();
                for point in rock.iter_mut() {
                    point.0 += highest_point + 4;
                    point.1 += 3;
                }
                new_rock = false;
            }
    
            // println!("ROCK AT {:?}", rock);
    
            // LEFT/RIGHT movement
            match c {
                '>' => {
                    // Can we move right?
                    let mut can_move = true;
                    for point in rock.iter() {
                        if point.1 + 1 >= right_wall {
                            can_move = false;
                        } else if points.contains(&(point.0, point.1 + 1)) {
                            can_move = false;
                        }
                    }
    
                    // println!("Pushing to the right - can move? {}", can_move);
                    if can_move {
                        for point in rock.iter_mut() {
                            point.1 += 1;
                        }
                    }
                },
                '<' => {
                    // Can we move left?
                    let mut can_move = true;
                    for point in rock.iter() {
                        if point.1 - 1 <= left_wall {
                            can_move = false;
                        } else if points.contains(&(point.0, point.1 - 1)) {
                            can_move = false;
                        }
                    }
                    // println!("Pushing to the left - can move? {}", can_move);
                    if can_move {
                        for point in rock.iter_mut() {
                            point.1 -= 1;
                        }
                    }
                },
                _ => (),
            }
    
            // FALLING DOWN
            let mut can_move_down = true;
            for point in rock.iter() {
                if point.0 - 1 <= floor {
                    can_move_down = false;
                    rock_stopped = true;
                } else if points.contains(&(point.0 - 1, point.1)) {
                    can_move_down = false;
                    rock_stopped = true;
                }
            }
    
            // println!("Moving down - can move? {}", can_move_down);
            if can_move_down {
                for point in rock.iter_mut() {
                    point.0 -= 1;
                }
            }
    
            if rock_stopped {
                // TODO - update the highest point
                for point in rock.iter() {
                    points.insert(*point);
                    if point.0 > highest_point {
                        highest_point = point.0
                    }
                }
                rock_n = (rock_n + 1) % rocks.len();
                count += 1;
                rock_stopped = false;
                new_rock = true;
                // println!("ROCK STOPPED AT {:?}", rock);
                // println!("NEW HIGHEST POINT: {}", highest_point);
    
                if count >= max_count {
                    println!("ENDING");
                    break;
                }
            }
        }
    }

    let answer1 = highest_point;

    println!("Part 1 answer: {}", answer1);

    println!("\n----- PART 2 -----\n");

    let answer2 = 0;

    println!("Part 2 answer: {}", answer2);
}
