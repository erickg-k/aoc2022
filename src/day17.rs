use std::collections::VecDeque;
use std::io;
use std::vec::Vec;

fn get_wind() -> Vec<bool> {
    for line in io::stdin().lines() {
        let line_str = line.expect("IO failed reading data");
        let wind: Vec<bool> = line_str.chars().map(|x| x == '<').collect();
        return wind;
    }
    vec![]
}

const WIDTH: usize = 7;
const NUM_ROCKS: usize = 2022;
const NUM_SPACE_BEFORE_ROCK: usize = 3;
const ROCK_START_LEFT_PADDING: usize = 2;

#[allow(dead_code, unused_imports)]
fn print_world(world: &World) {
    for layer in world.window.iter().rev() {
        print!("|");
        println!("{}|", layer.iter().collect::<String>());
    }
}

struct Rock {
    selected: usize,
    height: usize,
    width: usize,
    left: usize,
    bottom: usize,
}

struct World {
    window: VecDeque<Vec<char>>,
    rocks: Vec<Vec<Vec<char>>>,
    wind: Vec<bool>,
    rocks_heights: Vec<usize>,
    rocks_widths: Vec<usize>,
    chamber_height: usize,
    cur_wind_idx: usize,
    cur_rock_idx: usize,
    num_rock: usize,
}

impl World {
    pub fn new(wind: &Vec<bool>) -> Self {
        let rocks = vec![
            vec![vec!['#', '#', '#', '#']],
            vec![
                vec!['.', '#', '.'],
                vec!['#', '#', '#'],
                vec!['.', '#', '.'],
            ],
            vec![
                vec!['#', '#', '#'],
                vec!['.', '.', '#'],
                vec!['.', '.', '#'],
            ],
            vec![vec!['#'], vec!['#'], vec!['#'], vec!['#']],
            vec![vec!['#', '#'], vec!['#', '#']],
        ];
        let rocks_heights = rocks.iter().map(|rock| rock.len()).collect();
        let rocks_widths = rocks.iter().map(|rock| rock[0].len()).collect();
        Self {
            window: VecDeque::new(),
            rocks,
            wind: wind.clone(),
            chamber_height: 0,
            rocks_heights,
            rocks_widths,
            cur_wind_idx: 0,
            cur_rock_idx: 0,
            num_rock: 0,
        }
    }

    pub fn next_wind(&mut self) -> bool {
        let left = self.wind[self.cur_wind_idx];

        self.cur_wind_idx = (self.cur_wind_idx + 1) % self.wind.len();

        left
    }

    pub fn next_rock(&mut self) -> Rock {
        let selected = self.cur_rock_idx;
        let height = self.rocks_heights[selected];
        let width = self.rocks_widths[selected];
        let left = ROCK_START_LEFT_PADDING;
        let bottom = self.window.len() + NUM_SPACE_BEFORE_ROCK + self.chamber_height;

        self.cur_rock_idx = (self.cur_rock_idx + 1) % self.rocks.len();
        self.num_rock += 1;

        Rock {
            selected,
            left,
            height,
            width,
            bottom,
        }
    }
}

impl Rock {
    #[allow(dead_code, unused_imports)]
    pub fn new(world: &World, selected: usize) -> Self {
        Self {
            selected,
            left: ROCK_START_LEFT_PADDING,
            bottom: world.window.len() + NUM_SPACE_BEFORE_ROCK + world.chamber_height,
            height: world.rocks_heights[selected],
            width: world.rocks_widths[selected],
        }
    }
}

fn shall_push_left(world: &World, selected_rock: &Rock) -> bool {
    // will the rock hit boundary?
    if selected_rock.left <= 0 {
        return false;
    }
    // will the rock hit other things?
    let rock = &world.rocks[selected_rock.selected];
    for i in 0..selected_rock.height {
        // not in the window?
        if i + selected_rock.bottom >= world.window.len() + world.chamber_height {
            break;
        }

        for j in 0..selected_rock.width {
            if world.window[i + selected_rock.bottom - world.chamber_height]
                [selected_rock.left - 1 + j]
                == '#'
                && rock[i][j] == '#'
            {
                return false;
            }
        }
    }
    true
}

fn shall_push_right(world: &World, selected_rock: &Rock) -> bool {
    // will the rock hit boundary?
    if selected_rock.left + selected_rock.width >= WIDTH {
        return false;
    }
    let rock = &world.rocks[selected_rock.selected];

    // will the rock hit other things?
    for i in 0..selected_rock.height {
        // not in the window?
        if i + selected_rock.bottom >= world.window.len() + world.chamber_height {
            break;
        }

        for j in (0..selected_rock.width).rev() {
            if world.window[i + selected_rock.bottom - world.chamber_height]
                [selected_rock.left + 1 + j]
                == '#'
                && rock[i][j] == '#'
            {
                return false;
            }
        }
    }
    true
}

fn shall_fall(world: &World, selected_rock: &Rock) -> bool {
    // will the rock hit boundary?
    if selected_rock.bottom <= 0 {
        return false;
    }
    let rock = &world.rocks[selected_rock.selected];

    // will the rock hit other things?
    for i in 0..selected_rock.height {
        // not in the window?
        if i + selected_rock.bottom - 1 >= world.window.len() + world.chamber_height {
            break;
        }

        for j in 0..selected_rock.width {
            if world.window[i + selected_rock.bottom - world.chamber_height - 1]
                [selected_rock.left + j]
                == '#'
                && rock[i][j] == '#'
            {
                return false;
            }
        }
    }
    true
}

pub fn simulate_tetris() {
    let wind = get_wind();

    let mut world = World::new(&wind);
    while world.num_rock < NUM_ROCKS {
        let mut rock = world.next_rock();

        loop {
            let push_left = world.next_wind();
            if push_left {
                if shall_push_left(&world, &rock) {
                    rock.left -= 1;
                }
            } else {
                if shall_push_right(&world, &rock) {
                    rock.left += 1;
                }
            };

            if shall_fall(&world, &rock) {
                rock.bottom -= 1;
            } else {
                break;
            }
            // println!("\n{} rock: left={}, bottom={}", world.num_rock, rock.left, rock.bottom);
        }

        while world.window.len() < rock.bottom + rock.height - world.chamber_height {
            world.window.push_back(vec!['.'; WIDTH]);
        }
        let rock_form = &world.rocks[rock.selected];
        for i in 0..rock.height {
            for j in 0..rock.width {
                if rock_form[i][j] == '#' {
                    world.window[i + rock.bottom - world.chamber_height][j + rock.left] =
                        rock_form[i][j];
                }
            }
        }

        // println!("\n{} landed:", world.num_rock);
        // print_world(&world);
        // println!("{}", world.window.len());
    }

    println!("\n{} landed. size = {}", world.num_rock, world.window.len());
    // print_world(&world);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shall_push_left__in_window() {
        let wind = vec![true, true, true, true];
        let world = World::new(&wind);
        let rock = Rock::new(&world, 0);
        assert!(shall_push_left(&world, &rock));
    }

    #[test]
    fn test_shall_push_left__out_bound() {
        let wind = vec![true, true, true, true];
        let world = World::new(&wind);
        let mut rock = Rock::new(&world, 0);
        rock.left = 0;
        assert!(!shall_push_left(&world, &rock));
    }

    #[test]
    fn test_shall_push_left__when_collides() {
        let wind = vec![true, true, true, true];
        let mut world = World::new(&wind);
        world
            .window
            .push_back(vec!['#', '#', '.', '.', '.', '.', '.']);
        world
            .window
            .push_back(vec!['#', '.', '.', '.', '.', '.', '.']);

        let mut rock = Rock::new(&world, 1);
        rock.left = 1;
        rock.bottom = 0;
        assert!(!shall_push_left(&world, &rock));

        rock.left = 1;
        rock.bottom = 1;
        assert!(shall_push_left(&world, &rock));
    }

    #[test]
    fn test_shall_push_right__in_window() {
        let wind = vec![false; 4];
        let world = World::new(&wind);
        let mut rock = Rock::new(&world, 0);
        rock.left = 2;
        assert!(shall_push_right(&world, &rock));
    }

    #[test]
    fn test_shall_push_right__out_bound() {
        let wind = vec![false; 4];
        let world = World::new(&wind);
        let mut rock = Rock::new(&world, 0);
        rock.left = 3;
        assert!(!shall_push_right(&world, &rock));
    }

    #[test]
    fn test_shall_push_right__when_collides() {
        let wind = vec![false; 4];
        let mut world = World::new(&wind);
        world
            .window
            .push_back(vec!['.', '.', '.', '.', '.', '#', '#']);
        world
            .window
            .push_back(vec!['.', '.', '.', '.', '.', '.', '#']);

        let mut rock = Rock::new(&world, 1);
        rock.left = 3;
        rock.bottom = 0;
        assert!(!shall_push_right(&world, &rock));

        rock.left = 3;
        rock.bottom = 1;
        assert!(shall_push_right(&world, &rock));

        rock.left = 4;
        rock.bottom = 1;
        assert!(!shall_push_right(&world, &rock));
    }

    #[test]
    fn test_shall_fall__in_window() {
        let wind = vec![false; 4];
        let world = World::new(&wind);
        let mut rock = Rock::new(&world, 1);
        rock.bottom = 1;
        assert!(shall_fall(&world, &rock));
    }

    #[test]
    fn test_shall_fall__out_bound() {
        let wind = vec![false; 4];
        let world = World::new(&wind);
        let mut rock = Rock::new(&world, 0);
        rock.bottom = 0;
        assert!(!shall_fall(&world, &rock));
    }

    #[test]
    fn test_shall_fall__when_collides() {
        let wind = vec![false; 4];
        let mut world = World::new(&wind);
        world
            .window
            .push_back(vec!['.', '.', '#', '.', '.', '.', '.']);
        let mut rock = Rock::new(&world, 1);
        rock.left = 1;
        rock.bottom = 1;
        assert!(!shall_fall(&world, &rock));

        rock.left = 0;
        rock.bottom = 1;
        assert!(shall_fall(&world, &rock));
    }
}
