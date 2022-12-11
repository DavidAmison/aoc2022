use utils::{parse_field_unwrap, parse_field, files};
use std::collections::HashMap;
use std::cell::RefCell;

#[derive(Debug)]
struct Folder {
    name: String,
    parent: String,
    files: Vec<File>,
    dirs: Vec<String>,
    size: u32
}

impl Folder {
    fn new(name: &str, parent: &str) -> Self {
        Self {
            name: name.to_string(),
            parent: parent.to_string(),
            files: vec!(),
            dirs: vec!(),
            size: 0,
        }
    }
}

#[derive(Debug)]
struct File {
    name: String,
    size: u32
}

#[derive(Debug)]
enum Command {
    CD(String),
    LS,
}

impl std::str::FromStr for Command {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cmd = parse_field!(s => String, " " | String, " " | String, "");
        match (cmd.1, cmd.2) {
            (Some(_), Some(arg)) => Ok(Self::CD(arg)),
            (None, Some(_)) => Ok(Self::LS),
            (_, _) => Err("Unable to parse command"),
        }
    }
}


impl File {
    fn new(name: &str, size: u32) -> Self {
        Self {
            name: name.to_string(),
            size
        }
    }
}

impl std::str::FromStr for File {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_field!(s => u32, " " | String, "") {
            (Some(size), Some(name)) => {
                Ok(Self::new(&name, size))
            }
            (_, _) => {
                Err("File details not found")
            }
        }
    }
}

fn main() {
    let values = files::read_in_lines("input");

    println!("\n----- PART 1 -----\n");

    let mut dir_stack: Vec<String> = vec!("/".to_string());
    let mut dirs: HashMap<String, RefCell<Folder>> = HashMap::new();
    dirs.insert("/".to_string(), RefCell::new(Folder::new("/", "/")));
    // Create the tree
    for line in values.iter() {
        if line.chars().nth(0).unwrap() == '$' {
            let cmd = line.parse::<Command>().unwrap();
            match cmd {
                Command::CD(d) => {
                    if d.eq("..") {
                        dir_stack.pop();
                    } else {
                        let parent_name: String = dir_stack.join("/");
                        dir_stack.push(d);
                        let dir_name = dir_stack.join("/");
                        {
                            let mut parent = dirs.get(&parent_name).unwrap().borrow_mut();
                            parent.dirs.push(dir_name.clone());
                        }

                        dirs.entry(dir_name.clone())
                            .or_insert(RefCell::new(
                                Folder::new(&dir_name, &parent_name)
                            ));

                    }
                }
                Command::LS => {
                    // DO NOTHING
                    ()
                }
            }
        } else {
            // Must be listing contents!
            match line.parse::<File>() {
                Ok(f) => {
                    // Current folder
                    let current_name = dir_stack.join("/");
                    match dirs.get_mut(&current_name) {
                        Some(d) => d.borrow_mut().files.push(f),
                        _ => println!("Cannot find {}", current_name),
                    }
                    // folder.files.push(f);
                }
                _ => (),
            }

        }
    }

    // println!("{:?}", dirs);

    // Now traverse the tree and sim to get folder sized
    let total = calculate_folder_size(&dirs, &"/".to_string());

    let answer1: u32 = dirs.values().filter_map(
        |dir| if dir.borrow().size < 100000 {
            Some(dir.borrow().size)
        } else {
            None
        }
    ).sum();

    // for dir in dirs.values() {
        // println!("{} -> {}", dir.borrow().name, dir.borrow().size);
    // }

    println!("Part 1 answer: {}", answer1);

    println!("\n----- PART 2 -----\n");

    let total_disk = 70000000;
    let needed_space = 30000000;
    let currently_used_space = total;

    let minimum_to_delete = needed_space - (total_disk - currently_used_space);

    let answer2: u32 = dirs.values().filter_map(
        |dir| if dir.borrow().size > minimum_to_delete {
            Some(dir.borrow().size)
        } else {
            None
        }
    ).min()
    .unwrap();

    println!("Part 2 answer: {}", answer2);
}

fn calculate_folder_size(dirs: & HashMap<String, RefCell<Folder>>, folder: &String) -> u32 {
    let mut current = dirs.get(folder).unwrap().borrow_mut();
    let mut total = 0;
    for sub in current.dirs.iter() {
        total += calculate_folder_size(dirs, sub);
    }
    for file in current.files.iter() {
        total += file.size;
    }
    current.size = total;
    total
}
