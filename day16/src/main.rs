use utils::{parse_field_unwrap, parse_field, files};
use std::collections::HashMap;

type Row = (String, String, u32, String);

#[derive(Debug)]
struct Valve {
    id: String,
    flow_rate: u32,
    state: State,
    connected: Vec<String>,
}

#[derive(Debug)]
enum State {
    Open,
    Closed,
}

#[derive(Debug)]
enum Dijkstra {
    Unvisited,
    Tentative(i32),
    Visited(i32),
}

impl Dijkstra {
    fn unwrap(&self) -> i32 {
        match self {
            Self::Unvisited => 0,
            Self::Tentative(x) => *x,
            Self::Visited(x) => *x,
        }
    }

    fn visit(&mut self) {
        match self {
            Self::Unvisited => (),
            Self::Tentative(x) => *self = Self::Visited(*x),
            Self::Visited(_) => (),
        }
    }
}

impl Valve {
    fn new(id: String, rate: u32, connected_list: String) -> Self {
        Self {
            id,
            flow_rate: rate,
            state: State::Closed,
            connected: connected_list.split(", ").map(|s| s.to_string()).collect::<Vec<String>>(),
        }
    }

    fn is_closed(&self) -> bool {
        match self.state {
            State::Closed => true,
            State::Open => false,
        }
    }

    fn flow_rate(&self) -> u32 {
        self.flow_rate
    }
}

// Very inefficient implementation of Dijkstra's!
//
// Return Valve, flow, remaining_time
fn most_effective_moves(map: &HashMap<String, Valve>, position: &str, remaining_time: i32) -> Vec<(String, i32, i32)> {
    // Make a list of all nodes
    let mut dj = map.iter().map(|(k, v)| (k.to_string(), Dijkstra::Unvisited)).collect::<HashMap<String, Dijkstra>>();
    // Find current node
    dj.insert(position.to_string(), Dijkstra::Tentative(remaining_time));

    // for valve in map.iter() {
    //     println!("{:?}", valve);
    // }

    while let Some(next_node) = next_to_visit(&dj) {
        // println!("TRYING TO VISIT {}", next_node);
        // Do the algorithm
        for neighbour in map.get(&next_node).unwrap().connected.iter() {
            let current = dj.get(&next_node).unwrap().unwrap();
            match dj.get_mut(neighbour) {
                Some(v) => {
                    match v {
                        Dijkstra::Tentative(v) if *v > 0 => {
                            if current > 0 && current - 1 > *v {
                                *v = current - 1;
                            }
                        }
                        Dijkstra::Tentative(v) => *v = 0,
                        Dijkstra::Unvisited => {
                            if current > 0 {
                                *v = Dijkstra::Tentative(current - 1);
                            } else {
                                Dijkstra::Tentative(0);
                            }
                        }
                        Dijkstra::Visited(_) => (),
                    }
                },
                None => (),
            }
        }
        dj.get_mut(&next_node).unwrap().visit();
    }

    let mut result = dj.iter()
        .map(|(k, v)| (k.to_string(), map.get(k).unwrap().flow_rate() as i32 * (v.unwrap() as i32 - 1), v.unwrap() as i32 - 1))
        .filter(|(k, _, _)| map.get(k).unwrap().is_closed())
        .collect::<Vec<(String, i32, i32)>>();
    result.sort_by(|l, r| l.1.cmp(&r.1));
    result.reverse();

    result
}

fn next_to_visit(map: &HashMap<String, Dijkstra>) -> Option<String> {
    let mut max = 0;
    let mut to_visit = None;
    for (node, value) in map.iter() {
        if let Dijkstra::Tentative(value) = value {
            if *value > max {
                max = *value;
                to_visit = Some(node.to_string());
            }
        }
    }
    to_visit
}

fn main() {
    let mut values = files::read_in_lines("input")
        .iter()
        .map(|l| parse_field_unwrap!(l => String, " " | String, " has flow rate=" | u32, "; tunnels lead to valves " | String, ""))
        .map(|(_, id, flow, connected)| (id.clone(), Valve::new(id.clone(), flow, connected)))
        .collect::<HashMap<String, Valve>>();

    // for row in values.iter() {
    //     println!("{:?}", row);
    // }

    // The general approach is to consider only movement from current position to all other valves
    // that are worth opening (i.e. have flow rate that is not zero).
    // That narrows down the search space a little and then we search over that whole space to
    // find the best route through all those valves.
    // 
    // Dijkstras is used to compute distance to all nodes from the current position and then we order
    // them by effectiveness (i.e. how much pressure that move would release total). This could be
    // implemented A LOT more efficiently.
    //
    // At the moment that calculation is done EVERY time we want to check something. More efficient
    // MIGHT be to calculate a table/matrix of distances from each point to every other point,
    // then we can reduce the number of calculations. However, this method has worked well enough so
    // I am going to leave as-is.
    //
    // Part 2 uses the same method except each 'turn' either you OR the elephant moves (whoever has more
    // time on the clock). The search space had to be artificially narrowed by only checking the first n
    // possible moves rather than the full set (the maximum of remaining_time/2 and 3). This doesn't guarantee
    // the best solution and still takes a long while to run - but since we can submit multiple solutions and
    // local maximum are printed regularly I can submit multiple times until the correct answer is found.

    println!("\n----- PART 1 -----\n");

    let answer1 = solve(&mut values, "AA", 30);

    println!("Part 1 answer: {}", answer1);

    println!("\n----- PART 2 -----\n");

    // We cheat a little in part 2 by artificially narrowing the search space so it is not guaranteed to get
    // the BEST answer - but it will probably be pretty close and might just be good enough.
    let answer2 = solve_double(&mut values, "AA", "AA", 26, 26);

    println!("Part 2 answer: {}", answer2);
}

fn solve(map: &mut HashMap<String, Valve>, start: &str, time: i32) -> i32 {
    let mut max_release = 0;
    let mut release = 0;
    for m in most_effective_moves(map, start, time).iter() {
        // Only if we actually release more do we care
        if m.1 <= 0 {
            break;
        }

        map.get_mut(&m.0).unwrap().state = State::Open;
        release += m.1;
        let sub = solve(map, &m.0, m.2);
        release += sub;
        if release > max_release {
            max_release = release;
        }
        release -= sub;
        release -= m.1;
        map.get_mut(&m.0).unwrap().state = State::Closed;
    }
    max_release
}

fn solve_double(map: &mut HashMap<String, Valve>, me: &str, elephant: &str, my_time: i32, elephant_time: i32) -> i32 {
    let mut max_release = 0;
    let mut release = 0;
    if my_time >= elephant_time {
        // I move
        for m in most_effective_moves(map, me, my_time).iter().take(std::cmp::max(my_time/2, 3) as usize) {
            // Only if we actually release more do we care
            if m.1 <= 0 {
                break;
            }
            // println!("Me Opening {}", m.0);

            map.get_mut(&m.0).unwrap().state = State::Open;
            release += m.1;
            let sub = solve_double(map, &m.0, elephant, m.2, elephant_time);
            release += sub;
            if release > max_release {
                // println!("Found new max: {}", release);
                max_release = release;
            }
            release -= sub;
            release -= m.1;
            map.get_mut(&m.0).unwrap().state = State::Closed;

            if my_time == 26 {
                println!("SEARCHED ONE STARTINGM MOVE - current max = {}", max_release);
            }
        }
    } else {
        // Elephant moves
        for m in most_effective_moves(map, elephant, elephant_time).iter().take(std::cmp::max(elephant_time/2, 3) as usize) {
            // Only if we actually release more do we care
            if m.1 <= 0 {
                break;
            }
            // println!("Elephant Opening {}", m.0);

            map.get_mut(&m.0).unwrap().state = State::Open;
            release += m.1;
            let sub = solve_double(map, me, &m.0, my_time, m.2);
            release += sub;
            if release > max_release {
                // println!("Found new max: {}", release);
                max_release = release;
            }
            release -= sub;
            release -= m.1;
            map.get_mut(&m.0).unwrap().state = State::Closed;
        }
        if my_time == 26 {
            println!("SEARCHED ONE STARTINGM MOVE - current max = {}", max_release);
        }
    }
    max_release
}
