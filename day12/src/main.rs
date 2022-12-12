use utils::{parse_field_unwrap, files};

#[derive(Debug)]
enum Dijkstra {
    Unvisited,
    Tentative(u32),
    Visited(u32),
}

impl Dijkstra {
    fn unwrap(&self) -> u32 {
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

fn main() {
    // Read in characters and convert to numbers
    // Start = 0, End = 27
    // a - z = 1 - 26
    let mut map = files::read_in_matrix_as::<char>("input")
        .iter()
        .map(|c| {
            c.iter().map(|c| {
                if *c == 'S' { (1, Dijkstra::Tentative(0)) } else if *c == 'E' { (27, Dijkstra::Unvisited) } else { (*c as u32 - 96, Dijkstra::Unvisited) }
            }).collect::<Vec<(u32, Dijkstra)>>()
        })
        .collect::<Vec<Vec<(u32, Dijkstra)>>>();

    println!("\n----- PART 1 -----\n");

    // print_map(&map);

    let end = end_point(&map);
    // Should be same height as z!
    map[end.0][end.1].0 = 26;

    // Find the shortest path using dijkstra's
    let mut next = next_to_visit(&map);
    loop {
        for p in adjacent(next, &map) {
            // println!("Visiting ({}, {})", p.0, p.1);
            let mut tentative = map[next.0][next.1].1.unwrap() + 1;
            match map[p.0][p.1] {
                (_, Dijkstra::Tentative(x)) => {
                    if tentative > x {
                        tentative = x;
                    }
                }
                _ => (),
            }
            map[p.0][p.1].1 = Dijkstra::Tentative(tentative);
        }
        map[next.0][next.1].1.visit();
        // println!();
        // print_map(&map);

        next = next_to_visit(&map);

        if next == end {
            break;
        }
    }

    // println!("\n\n");
    // print_map(&map);

    println!("Part 1 answer: {:?}", map[next.0][next.1]);

    println!("\n----- PART 2 -----\n");

    // Reset the map
    for row in map.iter_mut() {
        for cell in row.iter_mut() {
            cell.1 = Dijkstra::Unvisited;
        }
    }

    // Set the starting point to the original end point
    map[end.0][end.1].1 = Dijkstra::Tentative(0);

    // Find the shortest path using dijkstra's
    let mut next = next_to_visit(&map);
    loop {
        for p in adjacent_part2(next, &map) {
            // println!("Visiting ({}, {})", p.0, p.1);
            let mut tentative = map[next.0][next.1].1.unwrap() + 1;
            match map[p.0][p.1] {
                (_, Dijkstra::Tentative(x)) => {
                    if tentative > x {
                        tentative = x;
                    }
                }
                _ => (),
            }
            map[p.0][p.1].1 = Dijkstra::Tentative(tentative);
        }
        map[next.0][next.1].1.visit();
        // println!();
        // print_map(&map);

        next = next_to_visit(&map);

        if map[next.0][next.1].0 == 1 {
            break;
        }
    }

    // print_map(&map);

    println!("Part 2 answer: {:?} & {:?}", map[next.0][next.1], next);
}

fn end_point(map: &Vec<Vec<(u32, Dijkstra)>>) -> (usize, usize) {
    for (i, row) in map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if cell.0 == 27 {
                return (i, j)
            }
        }
    }
    return (0, 0)
}

fn next_to_visit(map: &Vec<Vec<(u32, Dijkstra)>>) -> (usize, usize) {
    let mut next_to_visit = (0, 0);
    let mut next_tentative = 99999999;
    for (i, row) in map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            match cell.1 {
                Dijkstra::Tentative(x) => {
                    if x < next_tentative {
                        // println!("Found {} which is smaller than {} at {:?}", x, next_tentative, (i, j));
                        next_tentative = x;
                        next_to_visit = (i, j);
                    }
                }
                _ => (),
            }
        }
    }
    next_to_visit
}


fn adjacent(p: (usize, usize), map: &Vec<Vec<(u32, Dijkstra)>>) -> Vec<(usize, usize)> {
    let mut to_visit = vec!();
    let elevation = map[p.0][p.1].0;
    for (x, y) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        if p.0 as isize + x >= 0 &&
            p.0 as isize + x < map.len() as isize &&
            p.1 as isize + y >= 0 &&
            p.1 as isize + y < map[0].len() as isize
         {
            let xy = (p.0 as isize + x, p.1 as isize + y);
            // Tentative and unvisited if elevation is at most 1 higher
            match map[xy.0 as usize][xy.1 as usize] {
                (h, Dijkstra::Tentative(x)) if h <= elevation + 1 => to_visit.push((xy.0 as usize, xy.1 as usize)),
                (h, Dijkstra::Unvisited) if h <= elevation + 1 => to_visit.push((xy.0 as usize, xy.1 as usize)),
                _ => (),
            }
        }
    }
    to_visit
}

fn adjacent_part2(p: (usize, usize), map: &Vec<Vec<(u32, Dijkstra)>>) -> Vec<(usize, usize)> {
    let mut to_visit = vec!();
    let elevation = map[p.0][p.1].0;
    for (x, y) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        if p.0 as isize + x >= 0 &&
            p.0 as isize + x < map.len() as isize &&
            p.1 as isize + y >= 0 &&
            p.1 as isize + y < map[0].len() as isize
         {
            let xy = (p.0 as isize + x, p.1 as isize + y);
            // Tentative and unvisited if elevation is at most 1 lower (or many higher)
            match map[xy.0 as usize][xy.1 as usize] {
                (h, Dijkstra::Tentative(x)) if h >= elevation - 1 => to_visit.push((xy.0 as usize, xy.1 as usize)),
                (h, Dijkstra::Unvisited) if h >= elevation - 1 => to_visit.push((xy.0 as usize, xy.1 as usize)),
                _ => (),
            }
        }
    }
    to_visit
}

fn print_map(map: &Vec<Vec<(u32, Dijkstra)>>) {
    for row in map.iter() {
        for cell in row.iter() {
            print!("{:0>3}", cell.0);
            match cell.1 {
                Dijkstra::Unvisited => print!("(U-000) "),
                Dijkstra::Tentative(x) => print!("(T-{:0>3}) ", x),
                Dijkstra::Visited(x) => print!("(V-{:0>3}) ", x),
            }
            print!("")
        }
        println!();
    }
}