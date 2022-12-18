use std::cell::RefCell;
use std::collections::BinaryHeap;
use std::collections::VecDeque;
use std::fmt;
use std::io;
use std::vec::Vec;

#[derive(Debug, Clone)]
enum Operand {
    Variable,
    Value(i64),
}
#[derive(Debug, Clone)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

struct MonkeyMeta {
    items: VecDeque<i64>,
    operation: Operation,
    operands: [Operand; 2],
    test_divisor: i64,
    toss_to: [i64; 2],
    num_inspected: i64,
}

impl fmt::Display for MonkeyMeta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(items={{{}}},\n           operation={:?}, operands=[{:?},{:?}],\n           test_divisor={}, toss_to=[{},{}],\n           inspected={})",
            self.items
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(", "),
            self.operation,
            self.operands[0],
            self.operands[1],
            self.test_divisor,
            self.toss_to[0],
            self.toss_to[1],
            self.num_inspected
        )
    }
}

fn get_monkey_meta() -> Vec<RefCell<MonkeyMeta>> {
    let mut monkeys = Vec::new();

    let mut i = 0;
    let mut items = VecDeque::new();
    let mut test_divisor = 0;
    let mut test_true = 0;
    let mut test_false: i64;
    let mut operation = Operation::Add;
    let mut operands: [Operand; 2] = [Operand::Variable, Operand::Variable];
    for line in io::stdin().lines() {
        let line_str = line.expect("IO failed reading data");
        match i {
            1 => {
                let splits = line_str
                    .split(": ")
                    .last()
                    .expect("Got a list of items")
                    .split(",");
                items = VecDeque::from(
                    splits
                        .map(|s| s.trim().parse::<i64>().expect("a item number"))
                        .collect::<Vec<i64>>(),
                );
            }
            2 => {
                let ops: Vec<&str> = line_str
                    .split("=")
                    .last()
                    .expect("operation result")
                    .trim()
                    .split(" ")
                    .collect();
                // assume an `old` as an operan
                operands = [ops[0], ops[2]].map(|s| {
                    s.trim()
                        .parse::<i64>()
                        .map_or_else(|_| Operand::Variable, |v| Operand::Value(v))
                });
                operation = match ops[1] {
                    "*" => Operation::Multiply,
                    "/" => Operation::Divide,
                    "+" => Operation::Add,
                    "-" => Operation::Subtract,
                    _ => Operation::Add,
                };
            }
            3 => {
                test_divisor = line_str
                    .split(" ")
                    .last()
                    .expect("A divisor")
                    .parse::<i64>()
                    .expect("A number");
            }
            4 => {
                test_true = line_str
                    .split(" ")
                    .last()
                    .expect("to a monkey")
                    .parse::<i64>()
                    .expect("A number");
            }
            5 => {
                test_false = line_str
                    .split(" ")
                    .last()
                    .expect("to a monkey")
                    .parse::<i64>()
                    .expect("A number");
                monkeys.push(RefCell::new(MonkeyMeta {
                    items: items.clone(),
                    operation: operation.clone(),
                    operands: operands.clone(),
                    test_divisor,
                    toss_to: [test_false, test_true],
                    num_inspected: 0,
                }));
            }
            _ => {}
        }
        i = (i + 1) % 7;
    }

    monkeys
}

fn print_monkeys(monkeys: &Vec<RefCell<MonkeyMeta>>) {
    println!("monkeys: ");
    for (i, m) in monkeys.iter().enumerate() {
        println!("monkey {}: {}", i, m.borrow());
    }
}

const TEST_ROUNDS: u64 = 20;
const MANAGED_WORRY_LEVEL: i64 = 3;

fn simulate_round(monkeys: &Vec<RefCell<MonkeyMeta>>, managed_level: i64) {
    for monkey_cell in monkeys {
        let mut monkey = monkey_cell.borrow_mut();
        while let Some(item) = monkey.items.pop_front() {
            monkey.num_inspected += 1;
            let tmp_operands = monkey
                .operands
                .iter()
                .map(|o| match o {
                    Operand::Value(v) => *v,
                    Operand::Variable => item,
                })
                .collect::<Vec<i64>>();
            let result = match monkey.operation {
                Operation::Add => tmp_operands.iter().fold(0, |res, val| res + val),
                Operation::Subtract => tmp_operands[0] - tmp_operands[1],
                Operation::Multiply => tmp_operands.iter().fold(1, |res, val| res * val),
                Operation::Divide => tmp_operands[0] / tmp_operands[1],
            } / managed_level;
            let next_monkey_idx =
                monkey.toss_to[(result % monkey.test_divisor == 0) as usize] as usize;
            let mut next_monkey = monkeys[next_monkey_idx].borrow_mut();
            next_monkey.items.push_back(result);
        }
    }
}

pub fn get_two_most_active_monkey() {
    let monkeys = get_monkey_meta();
    print_monkeys(&monkeys);

    for i in 0..TEST_ROUNDS {
        simulate_round(&monkeys, MANAGED_WORRY_LEVEL);
        println!("\nAfter round {}:", i + 1);
        print_monkeys(&monkeys);
    }

    let mut heap = BinaryHeap::from(
        monkeys
            .iter()
            .map(|m| m.borrow().num_inspected)
            .collect::<Vec<i64>>(),
    );
    let result = heap.pop().unwrap() * heap.pop().unwrap();
    println!("{}", result)
}

fn get_number_pattern(monkeys: &Vec<RefCell<MonkeyMeta>>, monkey_idx: usize, item: i64) -> i64 {
    println!("{}: {}", monkey_idx, item);

    let mut cur_idx = monkey_idx;
    let mut cur_item = item;
    let mut history = Vec::new();
    history.push(cur_idx);
    loop {
        let monkey = monkeys[cur_idx].borrow();
        let tmp_operands = monkey
            .operands
            .iter()
            .map(|o| match o {
                Operand::Value(v) => *v,
                Operand::Variable => cur_item,
            })
            .collect::<Vec<i64>>();
        cur_item = match monkey.operation {
            Operation::Add => tmp_operands.iter().fold(0, |res, val| res + val),
            Operation::Subtract => tmp_operands[0] - tmp_operands[1],
            Operation::Multiply => tmp_operands.iter().fold(1, |res, val| res * val),
            Operation::Divide => tmp_operands[0] / tmp_operands[1],
        };
        cur_idx = monkey.toss_to[(cur_item % monkey.test_divisor == 0) as usize] as usize;
        history.push(cur_idx);

        // if cur_idx == monkey_idx {
        //     println!("{} = {}", cur_idx, cur_item);
        //     break;
        // }
        if cur_item >= f64::sqrt(i64::MAX as f64) as i64 {
            println!("{:?}", history);
            break;
        }
    }
    cur_item
}
// const TEST_MANY_ROUNDS: u64 = 10000;

pub fn get_two_most_active_monkey_many_rounds() {
    let monkeys = get_monkey_meta();
    print_monkeys(&monkeys);

    // let mut num_rounds = 0;
    // loop {
    //     simulate_round(&monkeys, 1);
    //     num_rounds += 1;
    //     println!("\nAfter round {}:", num_rounds);
    //     print_monkeys(&monkeys);
    //     if num_rounds > 5 {
    //         break;
    //     }
    // }

    // monkey 0: 80 is a special item
    // monkey 1: 56 is a special item
    // monkey 2: 52 is a special item
    for i in 0..monkeys.len() {
        println!("==========={i}============");
        for start_num in &monkeys[i].borrow().items {
            get_number_pattern(&monkeys, i, *start_num);
        }
    }
}
