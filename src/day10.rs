use std::io;
use std::vec::Vec;
#[derive(Debug)]
enum Instruction {
    Addx(i32),
    Noop,
}
fn get_instructions() -> Vec<Instruction> {
    let mut instructions = Vec::new();

    for line in io::stdin().lines() {
        let line_str = line.expect("IO failed reading data");
        if line_str == "noop" {
            instructions.push(Instruction::Noop);
        } else {
            let mut iter = line_str.as_str().split_whitespace().clone();
            iter.next().expect("Got addx");
            let moves = iter
                .next()
                .expect("Got moves")
                .parse::<i32>()
                .expect("Got number of moves");
            instructions.push(Instruction::Addx(moves));
        }
    }
    instructions
}

const FIRST_SUM_CYCLE: i32 = 20;
const SUM_EVERY_CYCLES: i32 = 40;
const NUM_CYCLES: i32 = 220;

fn get_instruction_duration(ins: &Instruction) -> i32 {
    match ins {
        Instruction::Addx(_) => 2,
        Instruction::Noop => 1,
    }
}

pub fn sum_signal_strength() {
    let instructions = get_instructions();
    let mut next_instruction = instructions.iter();

    let mut x = 1;
    let mut cycle = 1;
    let mut sum_strength = 0;
    let mut current_instruction = next_instruction.next().expect("Got an instruction");
    let mut cycles_duration = get_instruction_duration(&current_instruction);
    for _ in 0..NUM_CYCLES {
        if cycle % SUM_EVERY_CYCLES == FIRST_SUM_CYCLE {
            sum_strength += cycle * x;
        }
        // println!("cycles = {}: current_instruction = {:?}; x = {}", cycle, current_instruction, x);

        // calc the effect at the end of cycle
        cycles_duration -= 1;
        if cycles_duration <= 0 {
            if let Instruction::Addx(change) = current_instruction {
                x += change
            }
            current_instruction = next_instruction.next().unwrap_or(&Instruction::Noop);
            cycles_duration = get_instruction_duration(&current_instruction);
        }
        cycle += 1;
    }
    println!("sum_strength: {}", sum_strength);
}

const NEXT_LINE_EVERY_CYCLES: i32 = 40;

pub fn render_images() {
    let instructions = get_instructions();
    let mut next_instruction = instructions.iter();

    let mut screen = Vec::new();
    let mut x: i32 = 1;
    let mut cycle = 1;
    let mut current_row = Vec::new();
    let mut current_instruction = next_instruction.next().expect("Got an instruction");
    let mut cycles_duration = get_instruction_duration(&current_instruction);
    loop {
        // println!("cycles = {}: current_instruction = {:?}; x = {}", cycle, current_instruction, x);
        let rendered = if (x - 1..=x + 1).contains(&(current_row.len() as i32)) {
            '#'
        } else {
            '.'
        };
        current_row.push(rendered);
        if cycle % NEXT_LINE_EVERY_CYCLES == 0 {
            screen.push(current_row);
            current_row = Vec::new();
        }

        // calc the effect at the end of cycle
        cycles_duration -= 1;
        if cycles_duration <= 0 {
            if let Instruction::Addx(change) = current_instruction {
                x += change;
            }
            current_instruction = match next_instruction.next() {
                Some(ins) => ins,
                None => break,
            };
            cycles_duration = get_instruction_duration(&current_instruction);
        }
        cycle += 1;
    }
    println!("screen:");
    for row in screen {
        println!("{}", row.iter().collect::<String>());
    }
}
