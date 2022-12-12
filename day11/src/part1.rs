use utils::{parse_field_unwrap, files};

type MonkeyId = usize;
type Worry = usize;
type Item = usize;

#[derive(Debug)]
struct Monkey
{
    id: MonkeyId,
    items: Vec<Item>,
    op: fn(Worry) -> Worry,
    test: fn(Worry) -> MonkeyId,
    inspected: usize,
}

impl Monkey
{
    fn operation(&mut self, worry_level: Worry) -> Worry {
        self.inspected += 1;
        (self.op)(worry_level)
    }

    // Returns where the next item should be send
    fn test(&self, worry_level: Worry) -> MonkeyId {
        (self.test)(worry_level)
    }
}

pub fn part1() {
    // let values = files::read_in_lines("input");
    let mut monkeys = vec!(
        Monkey {
            id: 0,
            items: vec!(83, 88, 96, 79, 86, 88, 70),
            op: |worry| worry * 5,
            test: |worry| if worry % 11 == 0 { 2 } else { 3 },
            inspected: 0,
        },
        Monkey {
            id: 1,
            items: vec!(59, 63, 98, 85, 68, 72),
            op: |worry| worry * 11,
            test: |worry| if worry % 5 == 0 { 4 } else { 0 },
            inspected: 0,
        },
        Monkey {
            id: 2,
            items: vec!(90, 79, 97, 52, 90, 94, 71, 70),
            op: |worry| worry + 2,
            test: |worry| if worry % 19 == 0 { 5 } else { 6 },
            inspected: 0,
        },
        Monkey {
            id: 3,
            items: vec!(97, 55, 62),
            op: |worry| worry + 5,
            test: |worry| if worry % 13 == 0 { 2 } else { 6 },
            inspected: 0,
        },
        Monkey {
            id: 4,
            items: vec!(74, 54, 94, 76),
            op: |worry| worry * worry,
            test: |worry| if worry % 7 == 0 { 0 } else { 3 },
            inspected: 0,
        },
        Monkey {
            id: 5,
            items: vec!(58),
            op: |worry| worry + 4,
            test: |worry| if worry % 17 == 0 { 7 } else { 1 },
            inspected: 0,
        },
        Monkey {
            id: 6,
            items: vec!(66, 63),
            op: |worry| worry + 6,
            test: |worry| if worry % 2 == 0 { 7 } else { 5 },
            inspected: 0,
        },
        Monkey {
            id: 7,
            items: vec!(56, 56, 90, 96, 68),
            op: |worry| worry + 7,
            test: |worry| if worry % 3 == 0 { 4 } else { 1 },
            inspected: 0,
        }
    );

    println!("\n----- PART 1 -----\n");

    for round in 0..20 {
        for n in 0..monkeys.len() {
            for i in 0..monkeys[n].items.len() {
                let worry = monkeys[n].items.remove(0);
                let new = monkeys[n].operation(worry) / 3;
                let target = monkeys[n].test(new);
                // println!("Monkey {} throws {} (originally {}) to {}", n, new, worry, target);
                monkeys[target].items.push(new);
            }
        }
    }
    monkeys.sort_by(|m1, m2| m1.inspected.cmp(&m2.inspected));
    monkeys.reverse();
    for m in monkeys.iter() {
        println!("{}", m.inspected);
    }
    let answer1: usize = monkeys.iter().take(2).map(|m| m.inspected).product();

    println!("Part 1 answer: {}", answer1);
}
