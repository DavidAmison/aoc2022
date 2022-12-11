use utils::{parse_field_unwrap, files};

fn tree_is_visible_from(map: &Vec<Vec<isize>>, p: (usize, usize), dir: (isize, isize)) -> bool {
    let mut x = p.0 as isize;
    let mut y = p.1 as isize;
    let h = map[p.0][p.1];

    x -= dir.0;
    y -= dir.1;
    while x >= 0 && y >= 0 && x < map.len() as isize && y < map[0].len() as isize {
        if map[x as usize][y as usize] >= h {
            return false;
        }
        x -= dir.0;
        y -= dir.1;
    }
    true
}

fn viewing_distance_from_tree_in_direction(map: &Vec<Vec<isize>>, p: (usize, usize), dir: (isize, isize)) -> usize {
    let mut x = p.0 as isize;
    let mut y = p.1 as isize;
    let h = map[p.0][p.1];

    x -= dir.0;
    y -= dir.1;
    let mut dist = 0;
    while x >= 0 && y >= 0 && x < map.len() as isize && y < map[0].len() as isize {
        dist += 1;
        if map[x as usize][y as usize] >= h {
            break;
        }
        x -= dir.0;
        y -= dir.1;
    }
    dist
}

fn main() {
    let values = files::read_in_matrix_as::<isize>("input");

    println!("\n----- PART 1 -----\n");

    let x = values.len();
    let y = values[0].len();

    let mut count = 0;

    for i in 0..x {
        for j in 0..y {
            if tree_is_visible_from(&values, (i, j), (0, 1)) ||
                tree_is_visible_from(&values, (i, j), (0, -1)) ||
                tree_is_visible_from(&values, (i, j), (-1, 0)) ||
                tree_is_visible_from(&values, (i, j), (1, 0)) {
                    count += 1;
            }
        }
    }

    let answer1 = count;

    println!("Part 1 answer: {}", answer1);

    println!("\n----- PART 2 -----\n");

    let mut highest = 0;
    for i in 0..x {
        for j in 0..y {
            let score = viewing_distance_from_tree_in_direction(&values, (i, j), (0, 1)) *
                viewing_distance_from_tree_in_direction(&values, (i, j), (0, -1)) *
                viewing_distance_from_tree_in_direction(&values, (i, j), (-1, 0)) *
                viewing_distance_from_tree_in_direction(&values, (i, j), (1, 0));
            if score > highest {
                highest = score;
            }
        }
    }
    let answer2 = highest;

    println!("Part 2 answer: {}", answer2);
}
