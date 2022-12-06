use std::cmp;
use std::collections::BinaryHeap;
use std::io;

pub fn max_calories_elf() {
    let lines = io::stdin().lines();

    let mut max_cal = 0;
    let mut cur_cal = 0;
    for line in lines {
        match line
            .expect("IO failed reading data")
            .as_str()
            .parse::<i32>()
        {
            Ok(n) => {
                cur_cal += n;
            }
            Err(_) => {
                max_cal = cmp::max(max_cal, cur_cal);
                cur_cal = 0;
            }
        }
    }
    max_cal = cmp::max(max_cal, cur_cal);
    println!("Max cal: {max_cal}");
}

const TOP_3: usize = 3;

pub fn total_top_3_calories_elf() {
    let lines = io::stdin().lines();

    let mut heap = BinaryHeap::new();
    let mut cur_cal = 0;
    for line in lines {
        match line
            .expect("IO failed reading data")
            .as_str()
            .parse::<i32>()
        {
            Ok(n) => {
                cur_cal += n;
            }
            Err(_) => {
                heap.push(cur_cal);
                heap.shrink_to(TOP_3);
                cur_cal = 0;
            }
        }
    }
    let mut total_cal = 0;
    for _ in 0..TOP_3 {
        total_cal += heap.pop().expect("Top cal");
    }
    println!("Total cal: {total_cal}");
}
