use std::io;
use std::cmp;

pub fn max_calories_elf() {
    let lines = io::stdin()
        .lines();

    let mut max_cal = 0;
    let mut cur_cal = 0;
    for line in lines {
        match line.expect("IO failed reading data").as_str().parse::<i32>() {
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
