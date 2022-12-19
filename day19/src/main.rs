use utils::{parse_field_unwrap, files};
use std::collections::HashMap;
use std::collections::VecDeque;
use std::cmp;

#[derive(Debug, Clone, Copy)]
struct Factory {
    ore_robot_cost: usize,
    clay_robot_cost: usize,
    obsidian_robot_cost: (usize, usize),
    geode_robot_cost: (usize, usize),
}

impl Factory {
    fn can_build(&self, mat: Inventory, robot: Material) -> bool {
        match robot {
            Material::Ore => {
                self.ore_robot_cost <= mat.ore
            }
            Material::Clay => {
                self.clay_robot_cost <= mat.ore
            }
            Material::Obsidian => {
                self.obsidian_robot_cost.0 <= mat.ore &&
                self.obsidian_robot_cost.1 <= mat.clay
            }
            Material::Geode => {
                self.geode_robot_cost.0 <= mat.ore &&
                self.geode_robot_cost.1 <= mat.obsidian
            }
        }
    }

    fn could_build(&self, mat: Inventory, robot: Material) -> bool {
        match robot {
            Material::Ore => {
                self.ore_robot_cost <= mat.ore - mat.ore_robot
            }
            Material::Clay => {
                self.clay_robot_cost <= mat.ore- mat.ore_robot
            }
            Material::Obsidian => {
                self.obsidian_robot_cost.0 <= mat.ore - mat.ore_robot &&
                self.obsidian_robot_cost.1 <= mat.clay - mat.clay_robot
            }
            Material::Geode => {
                self.geode_robot_cost.0 <= mat.ore - mat.ore_robot &&
                self.geode_robot_cost.1 <= mat.obsidian - mat.obsidian_robot
            }
        }
    }

    fn should_build(&self, mat: Inventory, robot: Material, built: bool) -> bool {
        if self.could_build(mat, robot) && !built {
            false
        } else {
            match robot {
                Material::Ore => {
                    mat.ore_robot <= cmp::max(cmp::max(cmp::max(self.ore_robot_cost, self.clay_robot_cost), self.obsidian_robot_cost.0), self.geode_robot_cost.0)
                }
                Material::Clay => {
                    mat.clay_robot <= self.obsidian_robot_cost.1
                }
                _ => true,
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Clone, Copy)]
struct Inventory {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
    ore_robot: usize,
    clay_robot: usize,
    obsidian_robot: usize,
    geode_robot: usize,
}

impl Inventory {
    fn new() -> Self {
        Self {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
            ore_robot: 1,
            clay_robot: 0,
            obsidian_robot: 0,
            geode_robot: 0
        }
    }

    fn build(mut self, f: &Factory, robot: Material) -> Self {
       self = self.mine();
        match robot {
            Material::Ore => {
                self.ore -= f.ore_robot_cost;
                self.ore_robot += 1;
            }
            Material::Clay => {
                self.ore -= f.clay_robot_cost;
                self.clay_robot += 1;
            }
            Material::Obsidian => {
                self.ore -= f.obsidian_robot_cost.0;
                self.clay -= f.obsidian_robot_cost.1;
                self.obsidian_robot += 1;
            }
            Material::Geode => {
                self.ore -= f.geode_robot_cost.0;
                self.obsidian -= f.geode_robot_cost.1;
                self.geode_robot += 1;
            }
        }
        self
    }

    fn mine(mut self) -> Self {
        self.ore += self.ore_robot;
        self.clay += self.clay_robot;
        self.obsidian += self.obsidian_robot;
        self.geode += self.geode_robot;
        self
    }
}

fn main() {
    let values = files::read_in_lines("input")
        .iter()
        .map(|l| parse_field_unwrap!(l => String , ": Each ore robot costs " |
            usize , " ore. Each clay robot costs " |
            usize , " ore. Each obsidian robot costs " |
            usize , " ore and " |
            usize , " clay. Each geode robot costs " |
            usize , " ore and " |
            usize , " obsidian."))
        .map(|(_, ore, clay, obsidian1, obsidian2, geode1, geode2)| {
            Factory {
                ore_robot_cost: ore,
                clay_robot_cost: clay,
                obsidian_robot_cost: (obsidian1, obsidian2),
                geode_robot_cost: (geode1, geode2)}
            }
        ).collect::<Vec<Factory>>();

    // println!("{:?}", values[1]);

    println!("\n----- PART 1 -----\n");

    // let test1 = Factory {
    //     ore_robot_cost: 2,
    //     clay_robot_cost: 3,
    //     obsidian_robot_cost: (3, 8),
    //     geode_robot_cost: (3, 12)
    // };

    // let test2 = Factory {
    //     ore_robot_cost: 4,
    //     clay_robot_cost: 2,
    //     obsidian_robot_cost: (3, 14),
    //     geode_robot_cost: (2, 7)
    // };

    // let mut answer1 = search(&test1, 32);
    // println!("T1 => {}", answer1);
    // answer1 = search(&test2, 32);
    // println!("T2 => {}", answer1);

    let answer1: usize = values.iter().enumerate().map(|(i, f)| search(&f, 24) * (i + 1)).sum();

    println!("Part 1 answer: {}", answer1);

    println!("\n----- PART 2 -----\n");

    let answer2: usize = values.iter().take(3).enumerate().map(|(_, f)| search(&f, 32)).product();

    println!("Part 2 answer: {}", answer2);
}

fn search(f: &Factory, t: i32) -> usize {
    let mut queue = VecDeque::new();

    queue.push_back((Inventory::new(), 0, false));

    let mut cache: HashMap<i32, usize> = HashMap::new();

    while let Some((i, time, built)) = queue.pop_front() {
        // println!("T = {}   I = {:?}", time, i);
        let &current_best = cache.get(&time).unwrap_or(&0);
        if i.geode < current_best {
            // Skip it because it's not good enough
            // println!("NOT WORTH IT AT {}", time);
            continue;
        }
        // Are we better?
        cache.insert(time, cmp::max(current_best, i.geode));
        if time == t {
            // This is the end
            continue;
        }

        if f.can_build(i, Material::Geode) {
            queue.push_back((i.build(f, Material::Geode), time+ 1 , true));
            continue;
        }
        // Don't build anything
        queue.push_back((i.mine(), time + 1, false));

        // Build robots if possible and should
        for robot in [Material::Obsidian, Material::Clay, Material::Ore] {
            if f.can_build(i, robot) && f.should_build(i, robot, built) {
                queue.push_back((i.build(f, robot), time + 1, true));
            }
        }
    }

    *cache.get(&t).unwrap()
}