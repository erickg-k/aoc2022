use crate::char_bins;
use crate::matrix;
use crate::matrix::Point;
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

pub fn min_steps_in_hill() {
    let (hillmap, start, end) = get_hillmap();
    let max_x = hillmap.len();
    let max_y = hillmap[0].len();
    let mut visited = vec![vec![false; max_y]; max_x];

    let mut steps = VecDeque::from([(start.clone(), 0)]);
    let mut result = i32::MIN;
    while let Some((cur, num_steps)) = steps.pop_front() {
        if cur == end {
            result = num_steps;
            break;
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
    println!("start: {:?}", start);
    println!("end: {:?}", end);
    println!("steps: {:?}", result);
}
