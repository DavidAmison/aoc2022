use utils::{files};

#[derive(Clone, Debug)]
enum Packet {
    Array(Vec<Packet>),
    Value(u32),
}

impl Packet {
    fn from(s: &str) -> Self {
        // println!("Creating packet from: {}", s);

        let mut p: Vec<Self> = vec!();
        let mut level = 0;
        let mut sub: String = String::new();
        let mut value: String = String::new();
        for c in s.chars() {
            match c {
                '[' => {
                    if level != 0 {
                        sub.push(c);
                    }
                    level += 1;
                    continue;
                }
                ']' => {
                    level -= 1;
                    if level == 0 {
                        let sub_packet = Packet::from(&sub);
                        // println!("Found sub-packet: {:?}", sub_packet);
                        p.push(sub_packet);

                        sub.clear();
                    } else {
                        sub.push(c);
                    }
                    continue;
                }
                ',' => {
                    if level == 0 {
                        if let Ok(v) = value.parse::<u32>() {
                            // println!("Found sub-value: {:?}", Self::Value(v));
                            p.push(Self::Value(v));

                            value.clear();
                        }
                    } else {
                        sub.push(c);
                    }
                    continue;
                }
                c if c.is_alphanumeric() => {
                    if level == 0 {
                        value.push(c);
                    } else {
                        sub.push(c);
                    }
                }
                c => {
                    panic!("Found unexpected character: {}!", c);
                }
            }
        }
        if value.len() > 0 {
            let v = Self::Value(value.parse::<u32>().unwrap());
            // println!("Found sub-value: {:?}", v);
            p.push(v);

            value.clear();
        }
        Self::Array(p)
    }

    fn to_array(&self) ->Option<Vec<Packet>> {
        match self {
            Self::Array(a) => Some(a.clone()),
            Self::Value(_) => None
        }
    }

    fn to_value(&self) -> Option<u32> {
        match self {
            Self::Array(_) => None,
            Self::Value(v) => Some(*v)
        }
    }
}

fn main() {
    let values = files::read_in_chunks("input");

    println!("\n----- PART 1 -----\n");

    let mut answer1 = 0;
    for (i, pair) in values.iter().enumerate() {
        // println!("{:?}", pair);
        // compare(pair[0].clone(), pair[1].clone());
        let n0 = pair[0].len();
        let n1 = pair[1].len();
        let p0 = Packet::from(&pair[0][1..n0-1]);
        let p1 = Packet::from(&pair[1][1..n1-1]);
        // println!("comparing:\n{:?}\n{:?}", pair[0], pair[1]);
        let result = compare(&p0, &p1);
        // println!("{:?}\n", result);
        match result {
            Compare::Correct => answer1 += i + 1,
            _ => (),
        };
        // break;
    }

    println!("Part 1 answer: {}", answer1);

    println!("\n----- PART 2 -----\n");

    let mut packets = files::read_in_lines("input")
        .iter()
        .filter(|l| !l.is_empty())
        .map(|l| Packet::from(l)).collect::<Vec<Packet>>();
    packets.push(Packet::from("[[6]]"));
    packets.push(Packet::from("[[2]]"));

    packets.sort_by(|p1, p2| match compare(p1, p2) {
        Compare::Correct => std::cmp::Ordering::Less,
        Compare::Wrong => std::cmp::Ordering::Greater,
        Compare::Unknown => std::cmp::Ordering::Equal,
    });

    let mut index1 = 0;
    let mut index2 = 1;
    for (i, p) in packets.iter().enumerate() {
        // print_packet(p);
        // println!();
        match compare(p, &Packet::from("[[6]]")) {
            Compare::Unknown => index1 = i + 1,
            _ => (),
        }

        match compare(p, &Packet::from("[[2]]")) {
            Compare::Unknown => index2 = i + 1,
            _ => (),
        }
    }

    let answer2 = index1 * index2;

    println!("Part 2 answer: {}", answer2);
}

fn print_packet(p: &Packet) {
    if let Some(a) = p.to_array() {
        print!("[");
        for s in a {
            match s {
                Packet::Array(a) => print_packet(&Packet::Array(a)),
                Packet::Value(v) => print!("{},", v),
            }
        }
        print!("],");
    } else {
        print!("{}", p.to_value().unwrap());
    }
}

#[derive(Debug)]
enum Compare {
    Unknown,
    Correct,
    Wrong
}

fn compare(p1: &Packet, p2: &Packet) -> Compare {
    // What are we comparing?
    // print!("COMPARING: ");
    // print_packet(p1);
    // print!(" -- WITH -- ");
    // print_packet(p2);
    // println!();
    match (p1, p2) {
        (Packet::Array(a1), Packet::Array(a2)) => {
            // Compare each element of the array
            for i in 0..a1.len() {
                if a2.len() <= i {
                    return Compare::Wrong;
                }
                match compare(&a1[i], &a2[i]) {
                    Compare::Unknown => continue,
                    result => return result,
                };
            }
            if a2.len() > a1.len() {
                return Compare::Correct;
            } else {
                return Compare::Unknown;
            }
        },
        (Packet::Array(a1), Packet::Value(v2)) => {
            // Convert value to array and compare
            match compare(&p1, &Packet::Array(vec!(Packet::Value(*v2)))) {
                Compare::Unknown => {
                    if a1.len() > 1 {
                        return Compare::Wrong;
                    } else {
                        return Compare::Unknown;
                    }
                }
                result => return result,
            };
        },
        (Packet::Value(v1), Packet::Array(a2)) => {
            // Convert value to array and compare
            match compare(&Packet::Array(vec!(Packet::Value(*v1))), &p2) {
                Compare::Unknown => {
                    if a2.len() > 1 {
                        return Compare::Correct;
                    } else {
                        return Compare::Unknown;
                    }
                }
                result => return result,
            };
        },
        (Packet::Value(v1), Packet::Value(v2)) => {
            // Compare values directly
            if v1 < v2 {
                // println!("V1 < V2");
                return Compare::Correct;
            } else if v1 > v2 {
                // println!("V1 > V2");
                return Compare::Wrong;
            } else {
                // println!("V1 == V2");
                return Compare::Unknown;
            }
        },
    }
}