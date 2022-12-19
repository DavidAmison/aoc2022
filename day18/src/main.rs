use utils::{parse_field_unwrap, files};
use std::collections::HashSet;

type XYZ = (i32, i32, i32);

fn main() {
    let mut values = files::read_in_csv_matrix_as::<i32>("input")
        .iter()
        .map(|v| (v[0], v[1], v[2]))
        .collect::<HashSet<XYZ>>();

    println!("\n----- PART 1 -----\n");

    let answer1: usize = values.iter().map(|p| free_sides(*p, &values)).sum();

    println!("Part 1 answer: {}", answer1);

    println!("\n----- PART 2 -----\n");

    let max_x = values.iter().map(|(x, _, _)| x).max().unwrap().clone();
    let max_y = values.iter().map(|(_, y, _)| y).max().unwrap().clone();
    let max_z = values.iter().map(|(_, _, z)| z).max().unwrap().clone();

    // This will find SINGLE surrounded points
    // What about larger pockets?
    for x in 0..max_x.abs() {
        for y in 0..max_y.abs() {
            for z in 0..max_z.abs() {
                // println!("CHECKING ({}, {}, {})", x, y, z);
                if !values.contains(&(x, y, z)) {
                    if let Some(pocket) = is_pocket((x, y, z), &values, (max_x, max_y, max_z)) {
                        // Found surrounded point
                        // println!("POINT {} {} {} is surrounded", x, y, z);
                        for space in pocket.iter() {
                            values.insert(*space);
                        }
                    }
                }
            }
        }
    }

    let answer2: usize = values.iter().map(|p| free_sides(*p, &values)).sum();


    println!("Part 2 answer: {:?}", answer2);
}

fn free_sides(point: XYZ, rock: &HashSet<XYZ>) -> usize {
    let points = [(point.0 + 1, point.1, point.2),
    (point.0 - 1, point.1, point.2),
    (point.0, point.1 + 1, point.2),
    (point.0, point.1 - 1, point.2),
    (point.0, point.1, point.2 + 1),
    (point.0, point.1, point.2 - 1)];

    points.iter().filter(|p| !rock.contains(p)).count()
}

fn get_free_sides(point: XYZ, rock: &HashSet<XYZ>) -> Vec<XYZ> {
    let points = vec!((point.0 + 1, point.1, point.2),
    (point.0 - 1, point.1, point.2),
    (point.0, point.1 + 1, point.2),
    (point.0, point.1 - 1, point.2),
    (point.0, point.1, point.2 + 1),
    (point.0, point.1, point.2 - 1));

    points.iter()
        .filter(|p| !rock.contains(p))
        // .filter(|(x, y, z)| x > 0 && x < max.0 && y > 0 && y < max.1 && z > 0 && z < max.2)
        .map(|p| *p)
        .collect()
}

fn is_pocket(point: XYZ, rock: &HashSet<XYZ>, max: XYZ) -> Option<HashSet<XYZ>> {
    let mut to_check: Vec<XYZ> = vec!(point);
    let mut pocket: HashSet<XYZ> = HashSet::new();
    pocket.insert(point);
    let mut found = 0;
    while let Some(p) = to_check.pop() {
        for free in get_free_sides(p, &rock) {
            if free.0 < 0 || free.0 > max.0 ||
                free.1 < 0 || free.1 > max.1 ||
                free.2 < 0 || free.2 > max.1 {
                    return None;
            } else if !pocket.contains(&free) {
                pocket.insert(free);
                to_check.push(free);
            }
        }
    }
    Some(pocket)
}