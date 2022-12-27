use crate::matrix::Point;
use std::cmp;
use std::io;
use std::vec::Vec;

const SOURCE: Point<usize> = Point::<usize> { x: 500, y: 0 };

const ROCK: i32 = 1;
const AIR: i32 = 0;

fn get_paths() -> (Vec<Vec<Vec<i32>>>, i32, i32) {
    let mut paths = Vec::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for line in io::stdin().lines() {
        let line_str = line.expect("IO failed reading data");
        let path: Vec<Vec<i32>> = line_str
            .split(" -> ")
            .map(|pair_str| {
                pair_str
                    .split(",")
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect()
            })
            .collect();
        for p in &path {
            max_x = cmp::max(max_x, p[0]);
            max_y = cmp::max(max_y, p[1]);
        }
        paths.push(path);
    }
    (paths, max_x, max_y)
}

// we have a transposed map for mem access purpose
fn get_ground_map() -> Vec<Vec<i32>> {
    let (paths, max_x, max_y) = get_paths();
    let mut ground_map = Vec::with_capacity((max_x + 1) as usize);
    for _ in 0..=max_x {
        ground_map.push(vec![AIR; (max_y + 1) as usize]);
    }
    for path in &paths {
        let mut prev = path.first().unwrap();
        for p in &path[1..] {
            if prev[0] == p[0] {
                for y in cmp::min(prev[1], p[1])..=cmp::max(prev[1], p[1]) {
                    ground_map[p[0] as usize][y as usize] = ROCK;
                }
            } else {
                for x in cmp::min(prev[0], p[0])..=cmp::max(prev[0], p[0]) {
                    ground_map[x as usize][p[1] as usize] = ROCK;
                }
            }
            prev = p;
        }
    }
    return ground_map;
}

const OFFSET_MULTIPLIER: i32 = 4;

fn get_actual_ground_map(offset_x: usize) -> (Vec<Vec<i32>>, Point<usize>) {
    let (paths, mut max_x, mut max_y) = get_paths();
    max_x += offset_x as i32 * OFFSET_MULTIPLIER;
    max_y += 2; // new rocks here.
    let mut offset_source = SOURCE.clone();
    offset_source.x += offset_x;

    let mut ground_map = Vec::with_capacity((max_x + 1) as usize);
    for _ in 0..=max_x {
        ground_map.push(vec![AIR; (max_y + 1) as usize]);
    }
    for path in &paths {
        let mut prev = path.first().unwrap();
        for p in &path[1..] {
            if prev[0] == p[0] {
                for y in cmp::min(prev[1], p[1])..=cmp::max(prev[1], p[1]) {
                    ground_map[p[0] as usize + offset_x][y as usize] = ROCK;
                }
            } else {
                for x in cmp::min(prev[0], p[0])..=cmp::max(prev[0], p[0]) {
                    ground_map[x as usize + offset_x][p[1] as usize] = ROCK;
                }
            }
            prev = p;
        }
    }
    for x in 0..=max_x {
        ground_map[x as usize][max_y as usize] = ROCK;
    }
    return (ground_map, offset_source);
}

const DIRECTIONS: [(i64, i64); 3] = [(0, 1), (-1, 1), (1, 1)];

fn simluate_sanddrop(ground_map: &mut Vec<Vec<i32>>) -> bool {
    let mut p = SOURCE.clone();
    loop {
        let mut moved = false;
        let mut fallthrough = false;
        for (dx, dy) in DIRECTIONS {
            let next = Point::<usize> {
                x: (p.x as i64 + dx) as usize,
                y: (p.y as i64 + dy) as usize,
            };
            if next.x >= ground_map.len() || next.y >= ground_map[0].len() {
                // out of bound
                fallthrough = true;
                continue;
            }
            if ground_map[next.x][next.y] != ROCK {
                p = next;
                moved = true;
                break;
            }
        }

        if !moved {
            // stable
            if fallthrough {
                return false;
            }
            ground_map[p.x][p.y] = ROCK;
            break;
        }
    }
    true
}

pub fn simulate_filled_sand() {
    let mut ground_map = get_ground_map();
    let mut sum = 0;
    loop {
        let success = simluate_sanddrop(&mut ground_map);
        if success {
            sum += 1;
        } else {
            break;
        }
    }
    println!("sum={:?}", sum)
}

fn simluate_actual_sanddrop(ground_map: &mut Vec<Vec<i32>>, source: &Point<usize>) -> bool {
    let mut p = source.clone();
    loop {
        let mut moved = false;
        for (dx, dy) in DIRECTIONS {
            let next = Point::<usize> {
                x: (p.x as i64 + dx) as usize,
                y: (p.y as i64 + dy) as usize,
            };
            if next.x >= ground_map.len() || next.y >= ground_map[0].len() {
                // out of bound
                continue;
            }
            if ground_map[next.x][next.y] != ROCK {
                p = next;
                moved = true;
                break;
            }
        }

        if !moved {
            // stable
            ground_map[p.x][p.y] = ROCK;
            break;
        }
    }
    p != (*source)
}

pub fn simulate_stable_sand() {
    let (mut ground_map, source) = get_actual_ground_map(1000);
    let mut sum = 1;
    loop {
        let success = simluate_actual_sanddrop(&mut ground_map, &source);
        if success {
            sum += 1;
        } else {
            break;
        }
    }
    println!("sum={:?}", sum)
}
