use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;
use std::process;

const WINDOW_SIZE: usize = 4;
const MESSAGE_WINDOW_SIZE: usize = 14;

pub fn first_marker() {
    for line in io::stdin().lines() {
        let line_str = line.expect("IO failed reading data");

        let mut window = VecDeque::new();
        let mut count = 0;
        for c in line_str.chars() {
            count += 1;
            window.push_back(c);
            if window.len() >= WINDOW_SIZE {
                let window_set: HashSet<char> = HashSet::from_iter(window.iter().cloned());
                if window_set.len() == WINDOW_SIZE {
                    println!("Marker: {count}");
                    process::exit(0);
                }
                window.pop_front().expect("Got the first element");
            }
        }
    }
}
pub fn first_marker_for_message() {
    for line in io::stdin().lines() {
        let line_str = line.expect("IO failed reading data");

        let mut window = VecDeque::new();
        let mut count = 0;
        for c in line_str.chars() {
            count += 1;
            window.push_back(c);
            if window.len() >= MESSAGE_WINDOW_SIZE {
                let window_set: HashSet<char> = HashSet::from_iter(window.iter().cloned());
                if window_set.len() == MESSAGE_WINDOW_SIZE {
                    println!("Marker: {count}");
                    process::exit(0);
                }
                window.pop_front().expect("Got the first element");
            }
        }
    }
}
