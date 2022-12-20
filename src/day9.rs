use crate::matrix;
use crate::matrix::Point;
use std::cmp;
use std::io;
use std::ops::Sub;
use std::vec::Vec;

impl<T: std::ops::Sub<Output = T>> Sub for Point<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

fn get_actions() -> (Vec<(matrix::Direction, i32)>, Vec<Vec<bool>>, (i32, i32)) {
    let mut actions = Vec::new();

    for line in io::stdin().lines() {
        let line_str = line.expect("IO failed reading data");
        let mut iter = line_str.as_str().split_whitespace().clone();
        let dir = iter.next().expect("Got direction");
        let moves = iter.next().expect("Got moves");

        let move_dir = match dir {
            "D" => matrix::Direction::Down,
            "U" => matrix::Direction::Up,
            "L" => matrix::Direction::Left,
            "R" => matrix::Direction::Right,
            _ => panic!("Unknown direction"),
        };
        let num_moves = moves.parse::<i32>().expect("Got number of moves");
        actions.push((move_dir, num_moves));
    }
    let mut x = 0;
    let mut y = 0;
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    for (dir, moves) in &actions {
        match dir {
            matrix::Direction::Down => y -= moves,
            matrix::Direction::Up => y += moves,
            matrix::Direction::Left => x -= moves,
            matrix::Direction::Right => x += moves,
        }
        min_x = cmp::min(min_x, x);
        max_x = cmp::max(max_x, x);
        min_y = cmp::min(min_y, y);
        max_y = cmp::max(max_y, y);
    }
    let states = (0..(max_x - min_x + 1))
        .map(|_| vec![false; (max_y - min_y + 1) as usize])
        .collect();
    return (actions, states, (min_x, min_y));
}

fn catch_up_to_head(tail: &Point<i32>, head: &Point<i32>) -> Point<i32> {
    let mut result = tail.clone();
    for (dx, dy) in matrix::DIRECTIONS_WITH_ADJACENCY_AND_DIAGONAL_AND_ORIGIN {
        if tail.x + dx == head.x && tail.y + dy == head.y {
            return result;
        }
    }
    let diff = head.clone() - tail.clone();
    if diff.x == 0 {
        result.y += diff.y / 2;
    } else if diff.y == 0 {
        result.x += diff.x / 2;
    } else if diff.x.abs() > diff.y.abs() {
        result.x += diff.x / 2;
        result.y += diff.y;
    } else if diff.x.abs() == diff.y.abs() {
        result.x += diff.x / 2;
        result.y += diff.y / 2;
    } else {
        result.x += diff.x;
        result.y += diff.y / 2;
    }
    result
}

pub fn sum_tail_visited() {
    let (actions, mut states, (min_x, min_y)) = get_actions();
    let mut head = Point::<i32> {
        x: 0 - min_x,
        y: 0 - min_y,
    };
    let mut tail = head.clone();
    states[tail.x as usize][tail.y as usize] = true;

    for (dir, moves) in &actions {
        // println!("{:?}->{} = {:?}", dir, moves, states);
        for _ in 0..*moves {
            match dir {
                matrix::Direction::Down => {
                    head.y -= 1;
                    tail = catch_up_to_head(&tail, &head);
                }
                matrix::Direction::Up => {
                    head.y += 1;
                    tail = catch_up_to_head(&tail, &head);
                }
                matrix::Direction::Left => {
                    head.x -= 1;
                    tail = catch_up_to_head(&tail, &head);
                }
                matrix::Direction::Right => {
                    head.x += 1;
                    tail = catch_up_to_head(&tail, &head);
                }
            }
            states[tail.x as usize][tail.y as usize] = true;
        }
    }

    let all_positions = states
        .iter()
        .map(|state| state.iter().fold(0, |sum, acc| sum + *acc as i32))
        .fold(0, |sum, acc| sum + acc);
    println!("{:#?}", all_positions);
}

const NUM_TAILS: usize = 9;

fn print_map(states: &Vec<Vec<bool>>, head: &Point<i32>, tails: &Vec<Point<i32>>) {
    let mut current: Vec<Vec<char>> = states
        .iter()
        .map(|sliced| sliced.iter().map(|_| '.').collect())
        .collect();
    for i in (0..tails.len()).rev() {
        let tail = tails[i].clone();
        current[tail.x as usize][tail.y as usize] =
            char::from_u32((i + 1) as u32 + '0' as u32).expect("Character conversion");
    }
    current[head.x as usize][head.y as usize] = 'H';
    let mut m = matrix::from_2d(&current);
    m = matrix::transpose(&m);

    for i in (0..m.shape[0]).rev() {
        let line = (0..m.shape[1])
            .map(|j| matrix::index(&m, &[i as usize, j as usize]))
            .collect::<String>();
        println!("{}", line);
    }
}

pub fn sum_last_tail_visited() {
    let (actions, mut states, (min_x, min_y)) = get_actions();
    let mut head = Point::<i32> {
        x: 0 - min_x,
        y: 0 - min_y,
    };
    let mut tails: Vec<Point<i32>> = (0..NUM_TAILS).map(|_| head.clone()).collect();
    states[head.x as usize][head.y as usize] = true;

    for (dir, moves) in &actions {
        // println!("{:?}->{} = {:?}", dir, moves, states);
        println!("== {:?} {} ==", dir, *moves);
        for _ in 0..*moves {
            let mut last_head = NUM_TAILS;
            match dir {
                matrix::Direction::Down => {
                    head.y -= 1;
                    for i in 0..tails.len() {
                        if last_head == NUM_TAILS {
                            tails[i] = catch_up_to_head(&tails[i], &head);
                            last_head = 0;
                        } else {
                            tails[i] = catch_up_to_head(&tails[i], &tails[last_head]);
                            last_head += 1;
                        }
                    }
                }
                matrix::Direction::Up => {
                    head.y += 1;
                    for i in 0..tails.len() {
                        if last_head == NUM_TAILS {
                            tails[i] = catch_up_to_head(&tails[i], &head);
                            last_head = 0;
                        } else {
                            tails[i] = catch_up_to_head(&tails[i], &tails[last_head]);
                            last_head += 1;
                        }
                    }
                }
                matrix::Direction::Left => {
                    head.x -= 1;
                    for i in 0..tails.len() {
                        if last_head == NUM_TAILS {
                            tails[i] = catch_up_to_head(&tails[i], &head);
                            last_head = 0;
                        } else {
                            tails[i] = catch_up_to_head(&tails[i], &tails[last_head]);
                            last_head += 1;
                        }
                    }
                }
                matrix::Direction::Right => {
                    head.x += 1;
                    for i in 0..tails.len() {
                        if last_head == NUM_TAILS {
                            tails[i] = catch_up_to_head(&tails[i], &head);
                            last_head = 0;
                        } else {
                            tails[i] = catch_up_to_head(&tails[i], &tails[last_head]);
                            last_head += 1;
                        }
                    }
                }
            }
            states[tails[NUM_TAILS - 1].x as usize][tails[NUM_TAILS - 1].y as usize] = true;
            // print_map(&states, &head, &tails);
            // println!("");
        }
        print_map(&states, &head, &tails);
        println!("");
    }

    let all_positions = states
        .iter()
        .map(|state| state.iter().fold(0, |sum, acc| sum + *acc as i32))
        .fold(0, |sum, acc| sum + acc);
    println!("{:#?}", all_positions);
}
