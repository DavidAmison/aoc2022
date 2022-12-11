use utils::files;

fn main() {
    let values = files::read_in_chunks_as::<u32>("input");

    // let fold_compare = |count, (x, y)| {
    //     count + if x > y { 1 } else { 0 }
    // };

    println!("\n----- PART 1 -----\n");

    // Concise solution
    let answer1: u32 = values.iter().map(|v| v.iter().sum()).max().unwrap();
    // let answer1 = answer;

    println!("Part 1 Answer: {:?}", answer1);

    println!("\n----- PART 2 -----\n");

    let mut v = values.iter().map(|v| v.iter().sum()).collect::<Vec<u32>>();
    v.sort();
    let answer2: u32 = v.iter().rev().take(3).sum();

    println!("Part 2 Answer: {}", answer2);
}
