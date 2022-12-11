use utils::{parse_field_unwrap, files};

fn main() {
    let values = files::read_in_lines("input")
        .iter()
        .map(|l| parse_field_unwrap!(l => String, " " | String, ""))
        .collect::<Vec<(String, String)>>();


    println!("\n----- PART 1 -----\n");

    let score1: i32 = values.iter().map(
        |r| play_round(r)
    ).sum();

    println!("Part 1 answer: {}", score1);

    println!("\n----- PART 2 -----\n");

    let score2: i32 = values.iter().map(
        |r| play_round2(r)
    ).sum();


    println!("Part 2 answer: {}", score2);
}

fn play_round(round: &(String, String)) -> i32 {
    let mut score: i32 = 0;

    // Value of selected
    score += match round.1.as_str() {
        "X" => {
            1 + match round.0.as_str() {
                "A" => 3,
                "B" => 0,
                "C" => 6,
                _ => panic!("UNRECOGNIZED"),
            }
        }
        "Y" => {
            2 + match round.0.as_str() {
                "A" => 6,
                "B" => 3,
                "C" => 0,
                _ => panic!("UNRECOGNIZED"),
            }
        },
        "Z" => {
            3 + match round.0.as_str() {
                "A" => 0,
                "B" => 6,
                "C" => 3,
                _ => panic!("UNRECOGNIZED"),
            }
        },
        _ => 0
    };

    return score;
}


// X = Lose
// Y = Draw
// Z = Win
fn play_round2 (round: &(String, String)) -> i32 {
    let mut score: i32 = 0;

    // Value of selected
    score += match round.1.as_str() {
        "X" => {
            0 + match round.0.as_str() {
                "A" => 3,
                "B" => 1,
                "C" => 2,
                _ => panic!("UNRECOGNIZED"),
            }
        }
        "Y" => {
            3 + match round.0.as_str() {
                "A" => 1,
                "B" => 2,
                "C" => 3,
                _ => panic!("UNRECOGNIZED"),
            }
        },
        "Z" => {
            6 + match round.0.as_str() {
                "A" => 2,
                "B" => 3,
                "C" => 1,
                _ => panic!("UNRECOGNIZED"),
            }
        },
        _ => 0
    };

    return score;
}
