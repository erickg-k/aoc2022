use crate::matrix;
use std::cmp;
use std::io;
use std::ops::Sub;
use std::vec::Vec;
#[derive(Clone, Debug)]
pub struct Point<T> {
    x: T,
    y: T,
}

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

fn catch_up_to_head(tail: &mut Point<i32>, head: &Point<i32>) {
    for (dx, dy) in matrix::DIRECTIONS_WITH_ADJACENCY_AND_DIAGONAL_AND_ORIGIN {
        if tail.x + dx == head.x && tail.y + dy == head.y {
            return;
        }
    }
    let diff = head.clone() - tail.clone();
    if diff.x == 0 {
        tail.y += diff.y / 2;
    } else if diff.y == 0 {
        tail.x += diff.x / 2;
    } else if diff.x.abs() > diff.y.abs() {
        tail.x += diff.x / 2;
        tail.y += diff.y;
    } else {
        tail.x += diff.x;
        tail.y += diff.y / 2;
    }
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
                    catch_up_to_head(&mut tail, &head);
                }
                matrix::Direction::Up => {
                    head.y += 1;
                    catch_up_to_head(&mut tail, &head);
                }
                matrix::Direction::Left => {
                    head.x -= 1;
                    catch_up_to_head(&mut tail, &head);
                }
                matrix::Direction::Right => {
                    head.x += 1;
                    catch_up_to_head(&mut tail, &head);
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
