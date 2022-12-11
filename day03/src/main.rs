use utils::{files};

fn main() {
    let values = files::read_in_matrix_as::<char>("input")
        .iter().map(|v| v.iter().map(|c| convert_abc_to_num(c)).collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();

    println!("\n----- PART 1 -----\n");

    let mut sum = 0;
    for l in values.iter() {
        for c in l.iter().take(l.len()/2) {
            if l[l.len()/2..].contains(c) {
                sum += c;
                break;
            }
        }
    }

    println!("Part 1 answer: {}", sum);


    println!("\n----- PART 2 -----\n");

    sum = 0;
    for i in (2..values.len()).step_by(3) {
        let mut a = values[i].clone();
        let mut b = values[i-1].clone();
        let mut c = values[i-2].clone();
        // Sort and remove duplicates from each backpack
        a.sort();
        a.dedup();
        b.sort();
        b.dedup();
        c.sort();
        c.dedup();

        // Combine the lists together and sort them
        a.append(&mut b);
        a.append(&mut c);
        a.sort();

        // Now find the character repeated three times
        let mut badge: u32 = 0;
        let mut count = 1;
        for c in a {
            if c == badge {
                count += 1;
            } else {
                badge = c;
                count = 1;
            }

            if count == 3 {
                sum += badge;
            }
        }
    }

    println!("Part 2 answer: {}", sum);
}

fn convert_abc_to_num(c: &char) -> u32 {
    if c.is_ascii_uppercase() {
        *c as u32 - 65 + 27
    } else if c.is_ascii_lowercase() {
        *c as u32 - 97 + 1
    } else {
        0
    }
}
