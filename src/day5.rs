use std::io;
use std::vec::Vec;

enum State {
    INITIAL,
    INSTRUCTION,
}

fn craete_stack(stacks: &mut Vec<Vec<char>>, stack_map: &mut Vec<String>) {
    let num_stacks_str = stack_map.pop().expect("Get the last line of things");
    let num_stacks = num_stacks_str.as_str().split_whitespace().count();
    for _ in 0..num_stacks {
        stacks.push(Vec::new());
    }
    while let Some(craete_line) = stack_map.pop() {
        for x in 0..num_stacks {
            if x * 4 + 1 >= craete_line.len() {
                break;
            }
            let c = craete_line.chars().nth(x * 4 + 1).unwrap();
            if c != ' ' {
                stacks[x].push(c);
            }
        }
    }
}

fn top_crates_in_stack(stacks: &Vec<Vec<char>>) -> String {
    let mut res = String::new();
    for stack in stacks {
        let item = stack.last().expect("Got an item");
        res.push(*item);
    }
    return res;
}

pub fn top_crate_after_moving() {
    let mut state = State::INITIAL;
    let mut stack_map = Vec::new();
    let mut stacks = Vec::new();

    for line in io::stdin().lines() {
        let line_str = line.expect("IO failed reading data");

        if line_str == "" {
            craete_stack(&mut stacks, &mut stack_map);
            state = State::INSTRUCTION;
            continue;
        }

        match state {
            State::INITIAL => {
                stack_map.push(line_str);
            }
            State::INSTRUCTION => {
                let c = line_str
                    .as_str()
                    .split_whitespace()
                    .map(|e| e.parse::<i32>())
                    .filter(|e| e.is_ok())
                    .map(|e| e.unwrap() as usize)
                    .collect::<Vec<_>>();
                let (num_move, crate_from_id, crate_to_id) = (c[0], c[1], c[2]);
                for _ in 0..num_move {
                    let crate_ = stacks[crate_from_id - 1].pop().unwrap();
                    stacks[crate_to_id - 1].push(crate_);
                }
            }
        }
    }
    let top_crates = top_crates_in_stack(&stacks);
    println!("Top crates: {top_crates}");
}

pub fn top_crate_after_moving_with_new_crane() {
    let mut state = State::INITIAL;
    let mut stack_map = Vec::new();
    let mut stacks = Vec::new();

    for line in io::stdin().lines() {
        let line_str = line.expect("IO failed reading data");

        if line_str == "" {
            craete_stack(&mut stacks, &mut stack_map);
            state = State::INSTRUCTION;
            continue;
        }

        match state {
            State::INITIAL => {
                stack_map.push(line_str);
            }
            State::INSTRUCTION => {
                let c = line_str
                    .as_str()
                    .split_whitespace()
                    .map(|e| e.parse::<i32>())
                    .filter(|e| e.is_ok())
                    .map(|e| e.unwrap() as usize)
                    .collect::<Vec<_>>();
                let (num_move, crate_from_id, crate_to_id) = (c[0], c[1], c[2]);
                let mut stage = Vec::new();
                for _ in 0..num_move {
                    let crate_ = stacks[crate_from_id - 1].pop().unwrap();
                    stage.push(crate_);
                }
                while let Some(c) = stage.pop() {
                    stacks[crate_to_id - 1].push(c);
                }
            }
        }
    }
    let top_crates = top_crates_in_stack(&stacks);
    println!("Top crates: {top_crates}");
}
