use utils::{parse_field_unwrap, files};
use std::cmp;

type Rule = (String, i64, i64, i64, i64);
fn main() {
    let values = files::read_in_lines("input")
        .iter()
        .map(|l| parse_field_unwrap!(l => String, "at x=" | i64, ", y=" | i64, ": closest beacon is at x=" | i64, ", y=" | i64, ""))
        .collect::<Vec<Rule>>();

    println!("\n----- PART 1 -----\n");

    let y = 2000000;

    let mut ranges: Vec<(i64, i64)> = vec!();

    for line in values.iter() {
        let d = (line.1 - line.3).abs() + (line.2 - line.4).abs();
        let r = d - (y - line.2).abs();
        if line.4 == y {
            println!("SENSOR AT {}", line.3);
        }
        if r >= 0 {
            let range = (line.1 - r, line.1 + r);
            ranges.push(range);
        }
    }

    ranges.sort_by(|l, r| l.0.cmp(&r.0));
    let mut combined_ranges: Vec<(i64, i64)> = vec!();

    let mut i = 0;
    let mut j = 0;
    while i < ranges.len() {
        if combined_ranges.len() == 0 {
            combined_ranges.push(ranges[i]);
        } else if combined_ranges[j].1 >= ranges[i].0 {
            combined_ranges[j].1 = cmp::max(ranges[i].1, combined_ranges[j].1);
        } else {
            combined_ranges.push(ranges[i]);
            j += 1;
        }
        i += 1;
    }

    // println!("RANGES: {:?}", ranges);
    // println!("RANGES: {:?}", combined_ranges);

    println!("Don't forget to exclude sensors!");
    println!("Part 1 answer: {:?}", 1 + combined_ranges[0].1 - combined_ranges[0].0);  // Inclusive

    println!("\n----- PART 2 -----\n");

    let min = 0;
    let max = 4000000;

    for r in min..=max {
        let ranges = get_ranges(&values, r, min, max);
        if ranges.len() > 1 {
            let x = ranges[0].1 + 1;
            let y = r;
            println!("Part 2 Answer: ({},{}) => {}", x, y, 4000000*x + y);
            return;
        }
    }

    println!("Part 2 answer: {}", "Failed to find answer...");
}


fn get_ranges(rules: &Vec<Rule>, row: i64, min: i64, max: i64) -> Vec<(i64, i64)> {
    let mut ranges: Vec<(i64, i64)> = vec!();

    for line in rules.iter() {
        let d = (line.1 - line.3).abs() + (line.2 - line.4).abs();
        let r = d - (row - line.2).abs();
        if r >= 0 {
            let range = (cmp::max(line.1 - r, min), cmp::min(line.1 + r, max));
            if range.0 <= range.1 {
                // We have some valid overlap with min and max area
                ranges.push(range);
            }
        }
    }

    ranges.sort_by(|l, r| l.0.cmp(&r.0));
    let mut combined_ranges: Vec<(i64, i64)> = vec!();

    let mut i = 0;
    let mut j = 0;
    while i < ranges.len() {
        if combined_ranges.len() == 0 {
            combined_ranges.push(ranges[i]);
        } else if combined_ranges[j].1 >= ranges[i].0 {
            combined_ranges[j].1 = cmp::max(ranges[i].1, combined_ranges[j].1);
        } else {
            combined_ranges.push(ranges[i]);
            j += 1;
        }
        i += 1;
    }

    // Clamp values
    combined_ranges
}
