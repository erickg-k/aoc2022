use crate::char_bins;
use crate::matrix;
use crate::matrix::Point;
use std::io;
use std::vec::Vec;
use std::collections::VecDeque;
fn get_hillmap() -> (Vec<Vec<usize>>, Point<usize>, Point<usize>) {
    let mut hillmap = Vec::new();

    for line in io::stdin().lines() {
        let line_str = line.expect("IO failed reading data");
        let row = line_str
            .chars()
            .map(|c| char_bins::remap_char_to_flatten_loc(c))
            .collect::<Vec<usize>>();
        hillmap.push(row);
    }

    let start_val = char_bins::remap_char_to_flatten_loc('S');
    let end_val = char_bins::remap_char_to_flatten_loc('E');
    let mut start = Point { x: 0, y: 0 };
    let mut end = start.clone();
    for x in 0..hillmap.len() {
        for y in 0..hillmap[0].len() {
            if hillmap[x][y] == start_val {
                start = Point { x, y };
                hillmap[x][y] = 0;
            } else if hillmap[x][y] == end_val {
                end = Point { x, y };
                hillmap[x][y] = char_bins::remap_char_to_flatten_loc('z')+1;
            }
        }
    }
    return (hillmap, start, end);
}

pub fn min_steps_in_hill() {
    let (hillmap, start, end) = get_hillmap();
    let max_x = hillmap.len();
    let max_y = hillmap[0].len();
    let mut min_steps = hillmap.clone();
    for x in 0..max_x {
        for y in 0..max_y {
            min_steps[x][y] = usize::MAX;
        }
    }

    let mut steps = VecDeque::new();
    steps.push_back(start.clone());
    min_steps[start.x][start.y] = 0;
    while let Some(cur) = steps.pop_front() {
        for (dx, dy) in matrix::DIRECTIONS_WITH_ADJACENCY {
            let next_unbound = Point::<i32> {
                x: cur.x as i32 + dx,
                y: cur.y as i32 + dy
            };
            if next_unbound.x < 0 || next_unbound.x >= hillmap.len() as i32 || next_unbound.y < 0 || next_unbound.y >= hillmap[0].len()  as i32{
                continue;
            }
            let next = Point::<usize> {
                x: next_unbound.x as usize,
                y: next_unbound.y as usize
            };
            let diff = hillmap[next.x][next.y] as i32 - hillmap[cur.x][cur.y] as i32;
            if (-26..=1).contains(&diff) && min_steps[next.x][next.y] > 1+ min_steps[cur.x][cur.y] {
                min_steps[next.x][next.y] = min_steps[cur.x][cur.y] + 1;
                if next != end {
                    steps.push_back(next.clone());
                }
                println!("next: {:?} = {} ({}); cur: {:?} = {} ({})", next, hillmap[next.x][next.y], min_steps[next.x][next.y],cur, hillmap[cur.x][cur.y], min_steps[cur.x][cur.y]);
            }
        }
    }
    println!("start: {:?}", start);
    println!("end: {:?}", end);
    println!("steps: {:?}", min_steps[end.x][end.y]);
}
