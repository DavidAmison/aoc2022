use utils::{parse_field_unwrap, files};


fn find_first_unique_substring(s: &String, n: usize) -> usize {
    for i in (n-1)..s.len() {
        let mut sub = s[i-(n-1)..=i].chars().collect::<Vec<char>>();
        sub.sort();
        sub.dedup();
        if sub.len() == n {
            println!("{} @ {}", &s[i-(n-1)..=i], i+1);
            return i+1;
        }
    }
    return 0;
}

fn main() {
    let line = files::read_in_line("input");

    println!("\n----- PART 1 -----\n");

    let answer1 = find_first_unique_substring(&line, 4);

    println!("Part 1 answer: {}", answer1);

    println!("\n----- PART 2 -----\n");

    let answer2 = find_first_unique_substring(&line, 14);;

    println!("Part 2 answer: {}", answer2);
}
