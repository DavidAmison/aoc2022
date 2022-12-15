use utils::{parse_field_unwrap, files};
use std::collections::HashMap;

fn main() {
    let values: Vec<Vec<(u32, u32)>> = files::read_in_lines("input").iter()
        .map(|l|
            l.split("->")
            .map(|c| parse_field_unwrap!(c.trim() => u32, "," | u32, ""))
            .collect::<Vec<(u32, u32)>>()
        )
        .collect();

    let mut points: HashMap<(u32, u32), char> = HashMap::new();
    for line in values.iter() {
        let mut last_point = (0, 0);
        for (i, xy) in line.iter().enumerate() {
            if i == 0 {
                points.insert(*xy, '#');
                last_point = *xy;
            } else {
                // println!("From {:?} to {:?}", last_point, *xy);
                for x in if last_point.0 < xy.0 { last_point.0..=xy.0 } else { xy.0..=last_point.0 } {
                    for y in if last_point.1 < xy.1 { last_point.1..=xy.1 } else { xy.1..=last_point.1 } {
                        // println!("Inserting {:?}", (x, y));
                        points.insert((x, y), '#');
                    }
                }
                last_point = *xy;
            }
        }
    }

    // Where is the sand coming from
    let source = (500, 0);

    // Lowest point
    let max_y = points.iter().map(|(k, _)| k.1).max().unwrap();

    println!("\n\nFurthest sand can fall is {}", max_y);

    println!("\n----- PART 1 -----\n");

    let mut count = 0;
    loop {
        let mut end = false;
        // Drop a block of sand
        let mut p = source;
        loop {
            if p.1 >= max_y {
                end = true;
                break;
            }
            else if !points.contains_key(&(p.0, p.1 + 1)) {
                // Down
                p = (p.0, p.1 + 1);
            } else if !points.contains_key(&(p.0 - 1, p.1 + 1)) {
                // Down Right
                p = (p.0 - 1, p.1 + 1);

            } else if !points.contains_key(&(p.0 + 1, p.1 + 1)) {
                // Down Left
                p = (p.0 + 1, p.1 + 1);
            } else {
                println!("Sand came to rest at {:?}", p);
                points.insert(p, 'o');
                count += 1;
                // We cannot fall any further
                break;
            }
        }
        if end {
            break;
        }
    }


    let answer1 = count;

    println!("Part 1 answer: {}", answer1);

    println!("\n----- PART 2 -----\n");

    // Keep dropping sand until the source is blocked - lowest at max_y + 1
    loop {
        let mut end = false;
        // Drop a block of sand
        let mut p = source;
        loop {
            if p.1 == max_y + 1 {
                println!("Sand came to rest at {:?}", p);
                points.insert(p, 'o');
                count += 1;
                break;
            }
            else if !points.contains_key(&(p.0, p.1 + 1)) {
                // Down
                p = (p.0, p.1 + 1);
            } else if !points.contains_key(&(p.0 - 1, p.1 + 1)) {
                // Down Right
                p = (p.0 - 1, p.1 + 1);

            } else if !points.contains_key(&(p.0 + 1, p.1 + 1)) {
                // Down Left
                p = (p.0 + 1, p.1 + 1);
            } else {
                // Source blocked
                println!("Sand came to rest at {:?}", p);
                points.insert(p, 'o');
                count += 1;
                if p.0 == source.0 && p.1 == source.1 {
                    println!("SOURCE BLOCKED");
                    end = true;
                }
                break;
            }
        }
        if end {
            break;
        }
    }


    // let answer2 = count;
    let answer2 = points.iter()
        .filter_map(|(k, v)| if *v == 'o' { Some('o') } else { None })
        .count();

    println!("Part 2 answer: {}", count);
}
