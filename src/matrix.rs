use std::cell::RefCell;
use std::clone::Clone;
use std::fmt;
use std::rc::Rc;
use std::vec::Vec;

#[allow(dead_code)]
pub const DIRECTIONS_WITH_ADJACENCY: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
#[allow(dead_code)]
pub const DIRECTIONS_WITH_ADJACENCY_AND_ORIGIN: [(i32, i32); 5] =
    [(1, 0), (0, 1), (-1, 0), (0, -1), (0, 0)];
#[allow(dead_code)]
pub const DIRECTIONS_WITH_ADJACENCY_AND_DIAGONAL: [(i32, i32); 8] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];
#[allow(dead_code)]
pub const DIRECTIONS_WITH_ADJACENCY_AND_DIAGONAL_AND_ORIGIN: [(i32, i32); 9] = [
    (1, 0),
    (0, 1),
    (-1, 0),
    (0, -1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
    (0, 0),
];

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

pub fn manhattan_distance_i64(this: &Point<i64>, other: &Point<i64>) -> i64 {
    i64::abs(this.x - other.x) + i64::abs(this.y - other.y)
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct Matrix<T> {
    data: Rc<RefCell<Vec<T>>>,
    start: usize,
    pub shape: Vec<i32>,
    pub strides: Vec<i32>, // not a byte stride but a unit given I have type here.
    back_strides: Vec<i32>,
}

impl<T: std::fmt::Display + Copy> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.shape.len() == 1 {
            write!(f, "[").map_err(|err| println!("{:?}", err)).ok();
            for i in 0..self.shape[0] {
                write!(f, "{}", index(&self, &[i as usize]))
                    .map_err(|err| println!("{:?}", err))
                    .ok();
            }
            writeln!(f, "]")
        } else if self.shape.len() == 2 {
            write!(f, "[").map_err(|err| println!("{:?}", err)).ok();
            for i in 0..self.shape[0] {
                if i > 0 {
                    write!(f, " ").map_err(|err| println!("{:?}", err)).ok();
                }
                write!(f, "[").map_err(|err| println!("{:?}", err)).ok();
                for j in 0..self.shape[1] - 1 {
                    write!(f, "{},", index(&self, &[i as usize, j as usize]))
                        .map_err(|err| println!("{:?}", err))
                        .ok();
                }
                writeln!(
                    f,
                    "{}],",
                    index(&self, &[i as usize, (self.shape[1] - 1) as usize])
                )
                .map_err(|err| println!("{:?}", err))
                .ok();
            }
            writeln!(f, "]")
        } else {
            write!(f, "")
        }
    }
}

fn get_back_strides(strides: &Vec<i32>, shape: &Vec<i32>) -> Vec<i32> {
    return strides
        .iter()
        .zip(shape)
        .map(|(&s, &dim)| s * (dim - 1))
        .collect();
}

#[allow(dead_code, unused_imports)]
pub fn from_1d<T: Clone>(data: Rc<RefCell<Vec<T>>>) -> Matrix<T> {
    return Matrix {
        data: Rc::clone(&data),
        start: 0,
        shape: vec![(*data).borrow().len() as i32],
        strides: vec![1],
        back_strides: vec![1],
    };
}

pub fn from_2d<T: Clone>(matrix: &Vec<Vec<T>>) -> Matrix<T> {
    let mut data = Vec::new();
    for row in matrix {
        data.extend_from_slice(&row);
    }
    let shape = vec![matrix.len() as i32, matrix[0].len() as i32];
    let strides = vec![matrix[0].len() as i32, 1];
    let back_strides = get_back_strides(&strides, &shape);
    return Matrix {
        data: Rc::new(RefCell::new(data)),
        start: 0,
        shape,
        strides,
        back_strides,
    };
}

pub fn index<T: Clone + Copy>(matrix: &Matrix<T>, indices: &[usize]) -> T {
    let index_num = matrix
        .strides
        .iter()
        .zip(indices)
        .map(|(&stride, &index)| stride as i32 * index as i32)
        .fold(matrix.start as i32, |sum, i| sum + i);
    return (*matrix.data).borrow()[index_num as usize];
}

pub fn fliplr<T: Clone>(matrix: &Matrix<T>) -> Matrix<T> {
    assert!(matrix.strides.len() >= 2);

    let mut strides = matrix.strides.to_vec();
    let last_stride = strides.pop().expect("Last dimension");
    strides.push(-1 * last_stride);
    let back_strides = get_back_strides(&strides, &matrix.shape);
    let last_dim = *matrix.shape.last().expect("Last dimension") as i32;
    let mut start = matrix.start;
    if last_stride < 0 {
        start = (start as i32 - last_stride * (last_dim - 1)) as usize;
    } else {
        start = (start as i32 + last_stride * (last_dim - 1)) as usize;
    }
    return Matrix {
        data: Rc::clone(&matrix.data),
        start,
        shape: matrix.shape.to_vec(),
        strides,
        back_strides,
    };
}

pub fn flipud<T: Clone>(matrix: &Matrix<T>) -> Matrix<T> {
    let mut strides = matrix.strides.to_vec();
    let first_stride = strides[0];
    strides[0] = -strides[0];
    let back_strides = get_back_strides(&strides, &matrix.shape);
    let first_dim = *matrix.shape.first().expect("first dimension") as i32;
    let mut start = matrix.start;
    if first_stride < 0 {
        start = (start as i32 - first_stride * (first_dim - 1)) as usize;
    } else {
        start = (start as i32 + first_stride * (first_dim - 1)) as usize;
    }
    return Matrix {
        data: Rc::clone(&matrix.data),
        start,
        shape: matrix.shape.to_vec(),
        strides,
        back_strides,
    };
}

#[allow(dead_code, unused_imports)]
pub fn flip<T: Clone>(matrix: &Matrix<T>) -> Matrix<T> {
    return fliplr(&flipud(&matrix));
}

pub fn transpose<T: Clone>(matrix: &Matrix<T>) -> Matrix<T> {
    let data = Rc::clone(&(*matrix).data);
    let strides = matrix.strides.iter().rev().cloned().collect();
    let shape = matrix.shape.iter().rev().cloned().collect();
    let back_strides = get_back_strides(&strides, &shape);
    return Matrix {
        data,
        start: matrix.start,
        shape,
        strides,
        back_strides,
    };
}

pub fn rot90<T: Clone>(matrix: &Matrix<T>, k: i64) -> Matrix<T> {
    let _k = k % 4;
    return match _k {
        0 => matrix.clone(),
        1 => transpose(&fliplr(&matrix)),
        2 => fliplr(&flipud(&matrix)),
        _ => fliplr(&transpose(&matrix)),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_2d() {
        let t = from_2d::<i32>(&vec![vec![1, 2, 3], vec![4, 5, 6]]);
        assert_eq!(index(&t, &[0 as usize, 0 as usize]), 1);
        assert_eq!(index(&t, &[1 as usize, 0 as usize]), 4);
        assert_eq!(t.shape[0], 2);
        assert_eq!(t.shape[1], 3);
        assert_eq!(t.strides[0], 3);
        assert_eq!(t.strides[1], 1);
    }

    #[test]
    fn test_from_1d() {
        let t = from_1d::<i32>(Rc::new(RefCell::new(vec![1, 2, 3, 4, 5, 6])));
        assert_eq!(index(&t, &[0 as usize]), 1);
        assert_eq!(index(&t, &[3 as usize]), 4);
        assert_eq!(t.shape[0], 6);
        assert_eq!(t.strides[0], 1);
    }

    #[test]
    fn test_index_access() {
        let t = from_2d::<i32>(&vec![vec![1, 2, 3], vec![4, 5, 6]]);
        assert_eq!(index(&t, &[0 as usize, 2 as usize]), 3);
        assert_eq!(index(&t, &[1 as usize, 0 as usize]), 4);
    }

    #[test]
    fn test_transpose() {
        let mut t = from_2d::<i32>(&vec![vec![1, 2, 3], vec![4, 5, 6]]);
        t = transpose(&t);
        assert_eq!(index(&t, &[0 as usize, 1 as usize]), 4);
        assert_eq!(index(&t, &[1 as usize, 0 as usize]), 2);
    }

    #[test]
    fn test_fliplr() {
        let mut t = from_2d::<i32>(&vec![vec![1, 2, 3], vec![4, 5, 6]]);
        t = fliplr(&t);
        assert_eq!(index(&t, &[0 as usize, 2 as usize]), 1);
        assert_eq!(index(&t, &[1 as usize, 2 as usize]), 4);
    }

    #[test]
    fn test_flipud() {
        let mut t = from_2d::<i32>(&vec![vec![1, 2, 3], vec![4, 5, 6]]);
        t = flipud(&t);
        assert_eq!(index(&t, &[0 as usize, 1 as usize]), 5);
        assert_eq!(index(&t, &[1 as usize, 2 as usize]), 3);
    }

    #[test]
    fn test_flip() {
        let mut t = from_2d::<i32>(&vec![vec![1, 2, 3], vec![4, 5, 6]]);
        t = flip(&t);
        assert_eq!(index(&t, &[0 as usize, 2 as usize]), 4);
        assert_eq!(index(&t, &[1 as usize, 0 as usize]), 3);
    }

    #[test]
    fn test_rot90_0_time() {
        let mut t = from_2d::<i32>(&vec![vec![1, 2, 3], vec![4, 5, 6]]);
        t = rot90(&t, 0);
        assert_eq!(index(&t, &[0 as usize, 1 as usize]), 2);
        assert_eq!(index(&t, &[1 as usize, 0 as usize]), 4);
    }

    #[test]
    fn test_rot90_1_time() {
        let mut t = from_2d::<i32>(&vec![vec![1, 2, 3], vec![4, 5, 6]]);
        t = rot90(&t, 1);
        assert_eq!(index(&t, &[0 as usize, 1 as usize]), 6);
        assert_eq!(index(&t, &[1 as usize, 0 as usize]), 2);
    }

    #[test]
    fn test_rot90_2_times() {
        let mut t = from_2d::<i32>(&vec![vec![1, 2, 3], vec![4, 5, 6]]);
        t = rot90(&t, 2);
        assert_eq!(index(&t, &[0 as usize, 1 as usize]), 5);
        assert_eq!(index(&t, &[1 as usize, 0 as usize]), 3);
    }

    #[test]
    fn test_rot90_3_times() {
        let mut t = from_2d::<i32>(&vec![vec![1, 2, 3], vec![4, 5, 6]]);
        t = rot90(&t, 3);
        assert_eq!(index(&t, &[0 as usize, 1 as usize]), 1);
        assert_eq!(index(&t, &[1 as usize, 0 as usize]), 5);
    }
}
