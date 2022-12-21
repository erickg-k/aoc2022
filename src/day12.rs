use crate::char_bins;
use crate::matrix;
use crate::matrix::Point;
use std::cmp;
use std::collections::VecDeque;
use std::io;
use std::vec::Vec;

fn get_hillmap() -> (Vec<Vec<i32>>, Point<usize>, Point<usize>) {
    let mut hillmap = Vec::new();

    for line in io::stdin().lines() {
        let line_str = line.expect("IO failed reading data");
        let row = line_str
            .chars()
            .map(|c| char_bins::remap_char_to_i32(c))
            .collect::<Vec<i32>>();
        hillmap.push(row);
    }

    let start_val = char_bins::remap_char_to_i32('S');
    let end_val = char_bins::remap_char_to_i32('E');
    let mut start = Point { x: 0, y: 0 };
    let mut end = start.clone();
    for x in 0..hillmap.len() {
        for y in 0..hillmap[0].len() {
            if hillmap[x][y] == start_val {
                start = Point { x, y };
                hillmap[x][y] = 0;
            } else if hillmap[x][y] == end_val {
                end = Point { x, y };
                hillmap[x][y] = char_bins::remap_char_to_i32('z');
            }
        }
    }
    return (hillmap, start, end);
}

fn get_min_steps(hillmap: &Vec<Vec<i32>>, start: &Point<usize>, end: &Point<usize>) -> i32 {
    let max_x = hillmap.len();
    let max_y = hillmap[0].len();
    let mut visited = vec![vec![false; max_y]; max_x];

    let mut steps = VecDeque::from([(start.clone(), 0)]);
    while let Some((cur, num_steps)) = steps.pop_front() {
        if cur == *end {
            return num_steps;
        }
        for (dx, dy) in matrix::DIRECTIONS_WITH_ADJACENCY {
            let x = cur.x as i32 + dx;
            let y = cur.y as i32 + dy;
            if x < 0 || x >= max_x as i32 || y < 0 || y >= max_y as i32 {
                continue;
            }
            let next = Point::<usize> {
                x: x as usize,
                y: y as usize,
            };
            if hillmap[next.x][next.y] <= hillmap[cur.x][cur.y] + 1 {
                if !visited[next.x][next.y] {
                    visited[next.x][next.y] = true;
                    steps.push_back((next.clone(), num_steps + 1));
                }
            }
        }
    }
    i32::MAX
}

pub fn min_steps_in_hill() {
    let (hillmap, start, end) = get_hillmap();
    let result = get_min_steps(&hillmap, &start, &end);
    println!("start: {:?}", start);
    println!("end: {:?}", end);
    println!("steps: {:?}", result);
}

// I am lazy
pub fn min_steps_from_a_in_hill() {
    let (mut hillmap, start, end) = get_hillmap();
    let max_x = hillmap.len();
    let max_y = hillmap[0].len();
    hillmap[start.x][start.y] = 200;

    let mut min_result = i32::MAX;
    for i in 0..max_x {
        for j in 0..max_y {
            if hillmap[i][j] == 0 {
                let start = Point {
                    x: i as usize,
                    y: j as usize,
                };
                min_result = cmp::min(get_min_steps(&hillmap, &start, &end), min_result);
                // println!("start: {:?} {:?}", start, min_result);
            }
        }
    }

    println!("steps: {:?}", min_result);
}
