use utils::{parse_field_unwrap, files};

type MonkeyId = usize;
type Worry = usize;
type Test = usize;

#[derive(Debug, Clone, Copy)]
enum Op {
    Mul(usize),
    Add(usize),
    Square,
}

#[derive(Debug, Clone)]
struct Item {
    initial_worry: usize,
    ops: Vec<Op>,
}

impl Item {
    fn new(worry: usize) -> Self {
        Self {
            initial_worry: worry,
            ops: vec!(),
        }
    }

    fn add_operation(&mut self, op: Op) {
        self.ops.push(op);
    }

    fn is_divisible_by(&self, x: usize) -> bool {
        let mut modulo = self.initial_worry % x;
        for op in self.ops.iter() {
            match op {
                Op::Add(y) => {
                    modulo = (modulo + y % x) % x;
                }
                Op::Mul(y) => {
                    modulo = (modulo * y % x) % x;
                }
                Op::Square => {
                    modulo = (modulo * modulo) % x;
                }
            }
        }
        modulo == 0
    }
}

#[derive(Debug)]
struct Monkey
{
    id: MonkeyId,
    items: Vec<Item>,
    op: Op,
    test: (Test, MonkeyId, MonkeyId),
    inspected: usize,
}


pub fn part2() {

    // let values = files::read_in_lines("input");
    let mut monkeys = vec!(
        Monkey {
            id: 0,
            items: vec!(Item::new(83), Item::new(88), Item::new(96), Item::new(79), Item::new(86), Item::new(88), Item::new(70)),
            op: Op::Mul(5),
            test: (11, 2, 3),
            inspected: 0,
        },
        Monkey {
            id: 1,
            items: vec!(Item::new(59), Item::new(63), Item::new(98), Item::new(85), Item::new(68), Item::new(72)),
            op: Op::Mul(11),
            test: (5, 4, 0),
            inspected: 0,
        },
        Monkey {
            id: 2,
            items: vec!(Item::new(90), Item::new(79), Item::new(97), Item::new(52), Item::new(90), Item::new(94), Item::new(71), Item::new(70)),
            op: Op::Add(2),
            test: (19, 5, 6),
            inspected: 0,
        },
        Monkey {
            id: 3,
            items: vec!(Item::new(97), Item::new(55), Item::new(62)),
            op: Op::Add(5),
            test: (13, 2, 6),
            inspected: 0,
        },
        Monkey {
            id: 4,
            items: vec!(Item::new(74), Item::new(54), Item::new(94), Item::new(76)),
            op: Op::Square,
            test: (7, 0, 3),
            inspected: 0,
        },
        Monkey {
            id: 5,
            items: vec!(Item::new(58)),
            op: Op::Add(4),
            test: (17, 7, 1),
            inspected: 0,
        },
        Monkey {
            id: 6,
            items: vec!(Item::new(66), Item::new(63)),
            op: Op::Add(6),
            test: (2, 7, 5),
            inspected: 0,
        },
        Monkey {
            id: 7,
            items: vec!(Item::new(56), Item::new(56), Item::new(90), Item::new(96), Item::new(68)),
            op: Op::Add(7),
            test: (3, 4, 1),
            inspected: 0,
        }
    );

    println!("\n----- PART 2 -----\n");

    for _ in 0..10000 {
        for n in 0..monkeys.len() {
            for i in 0..monkeys[n].items.len() {
                let mut item = monkeys[n].items.remove(0);
                let op = monkeys[n].op;
                let test = monkeys[n].test;
                item.add_operation(op);
                let index = if item.is_divisible_by(test.0) { test.1 } else { test.2 };
                monkeys[index].items.push(item);
                monkeys[n].inspected += 1;
            }
        }
    }
    let mut temp = monkeys.iter().map(|m| m.inspected).collect::<Vec<usize>>();
    temp.sort();
    temp.reverse();
    // monkeys.sort_by(|m1, m2| m1.inspected.cmp(&m2.inspected));
    // monkeys.reverse();
    for m in monkeys.iter() {
        println!("{}", m.inspected);
    }
    let answer1: usize = temp.iter().take(2).product();

    println!("Part 1 answer: {}", answer1);

    println!("\n----- PART 2 -----\n");

    let answer2 = 0;

    println!("Part 2 answer: {}", answer2);
}
