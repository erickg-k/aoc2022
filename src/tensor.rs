use std::clone::Clone;
use std::vec::Vec;

pub struct Tensor<T> {
    data: Vec<T>,
    shape: Vec<u32>,
    stride: Vec<u32>, // not a byte stride but a unit given I have type here.
}

pub fn from_1d<T: Clone>(data: &Vec<T>) -> Tensor<T> {
    return Tensor {
        data: data.to_vec(),
        shape: vec![data.len() as u32],
        stride: vec![1],
    };
}

pub fn from_2d<T: Clone>(matrix: &Vec<Vec<T>>) -> Tensor<T> {
    let mut data = Vec::new();
    for row in matrix {
        data.extend_from_slice(&row);
    }
    return Tensor {
        data,
        shape: vec![matrix.len() as u32, matrix[0].len() as u32],
        stride: vec![matrix[0].len() as u32, 1],
    };
}

// pub fn flip<T: Clone>(tensor: &Tensor<T>) -> Tensor<T> {
//     return from_2d::<T>(&vec![vec![1,2,3], vec![4,5,6]]);
// }

// pub fn transpose<T: Clone>(tensor: &Tensor<T>) -> Tensor<T> {
//     return from_2d::<T>(&vec![vec![1,2,3], vec![4,5,6]]);
// }

// pub fn rot90<T: Clone>(tensor: &Tensor<T>) -> Tensor<T> {
//     return from_2d::<T>(&vec![vec![1,2,3], vec![4,5,6]]);
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_2d() {
        let t = from_2d::<i32>(&vec![vec![1, 2, 3], vec![4, 5, 6]]);
        assert_eq!(t.data[0], 1);
        assert_eq!(t.data[3], 4);
        assert_eq!(t.shape[0], 2);
        assert_eq!(t.shape[1], 3);
        assert_eq!(t.stride[0], 3);
        assert_eq!(t.stride[1], 1);
    }

    #[test]
    fn test_from_1d() {
        let t = from_1d::<i32>(&vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(t.data[0], 1);
        assert_eq!(t.data[3], 4);
        assert_eq!(t.shape[0], 6);
        assert_eq!(t.stride[0], 1);
    }
}
