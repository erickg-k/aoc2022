use crate::char_bins;
use crate::matrix;
use std::cmp;
use std::io;
use std::vec::Vec;

fn get_treemap() -> Vec<Vec<usize>> {
    let mut treemap = Vec::new();

    for line in io::stdin().lines() {
        let line_str = line.expect("IO failed reading data");
        let row = line_str
            .chars()
            .map(|c| char_bins::remap_char_to_flatten_loc(c))
            .collect();
        treemap.push(row);
    }
    return treemap;
}

/// 4 scanning directions:
///   - max from top to bottom (get_treemap_masks_positive/false)
///   - max from left to right (get_treemap_masks_positive/true)
///   - max from bottom to top (get_treemap_masks_negative/false)
///   - max from right to left (get_treemap_masks_negative/true)
///
///   - increasing length from top to bottom (get_treemap_increasing_length_positive/false)
///   - increasing length from left to right (get_treemap_increasing_length_positive/true)
///   - increasing length from bottom to top (get_treemap_increasing_length_negative/false)
///   - increasing length from right to left (get_treemap_increasing_length_negative/true)
const DIRECTIONS: [(i64, i64); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn get_treemap_masks_positive(treemap: &Vec<Vec<usize>>, column: bool) -> Vec<Vec<usize>> {
    let direction = column as usize;
    let mut mask = treemap.clone();
    let len_row = treemap.len() as i64;
    let len_column = treemap[0].len() as i64;
    for i in DIRECTIONS[direction].0..len_row {
        for j in DIRECTIONS[direction].1..len_column {
            mask[i as usize][j as usize] = cmp::max(
                mask[i as usize][j as usize],
                mask[(i - DIRECTIONS[direction].0) as usize]
                    [(j - DIRECTIONS[direction].1) as usize],
            );
        }
    }
    return mask;
}

fn get_treemap_masks_negative(treemap: &Vec<Vec<usize>>, column: bool) -> Vec<Vec<usize>> {
    let direction = column as usize + 2;
    let mut mask = treemap.clone();
    let len_row = treemap.len() as i64;
    let len_column = treemap[0].len() as i64;
    for i in (0..len_row + DIRECTIONS[direction].0).rev() {
        for j in (0..len_column + DIRECTIONS[direction].1).rev() {
            mask[i as usize][j as usize] = cmp::max(
                mask[i as usize][j as usize],
                mask[(i - DIRECTIONS[direction].0) as usize]
                    [(j - DIRECTIONS[direction].1) as usize],
            );
        }
    }
    return mask;
}

const NUM_DIGITS: usize = 10;

fn get_least_equal_distances(treemap: &matrix::Matrix<usize>) -> Vec<Vec<usize>> {
    let last_dim = *treemap.shape.last().expect("Got last dimension");
    let first_dim = *treemap.shape.first().expect("Got first dimension");

    let mut distances_treemap = Vec::new();
    for i in 0..first_dim {
        let mut seen_index = [-1 as i64; NUM_DIGITS];
        let mut distances = vec![0 as usize];
        seen_index[matrix::index(&treemap, &[i as usize, 0 as usize])] = 0;
        for j in 1..last_dim {
            let ele = matrix::index(&treemap, &[i as usize, j as usize]);
            let mut distance: usize = j as usize;
            for seen_tree in ele..NUM_DIGITS {
                if seen_index[seen_tree] >= 0 {
                    distance = cmp::min(distance, (j as i64 - seen_index[seen_tree]) as usize);
                }
            }
            seen_index[ele] = j as i64;
            distances.push(distance);
        }
        distances_treemap.push(distances);
    }
    return distances_treemap;
}

fn reshape_treemap_boundary(treemap: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let len_row = treemap.len() as usize;
    let len_column = treemap[0].len() as usize;
    let mut new_treemap = Vec::new();
    for i in 0..len_row {
        let mut row = Vec::new();
        for j in 0..len_column {
            row.push(treemap[i][j] as usize);
        }
        new_treemap.push(row);
    }
    for j in 1..len_column - 1 {
        new_treemap[0][j] = cmp::max(new_treemap[0][j], new_treemap[1][j]);
        new_treemap[len_row - 1][j] =
            cmp::max(new_treemap[len_row - 1][j], new_treemap[len_row - 2][j]);
    }
    for i in 1..len_row - 1 {
        new_treemap[i][0] = cmp::max(new_treemap[i][0], new_treemap[i][1]);
        new_treemap[i][len_column - 1] = cmp::max(
            new_treemap[i][len_column - 1],
            new_treemap[i][len_column - 2],
        );
    }
    return new_treemap;
}

pub fn sum_visible_trees() {
    let treemap = get_treemap();
    let len_row = treemap.len();
    let len_column = treemap[0].len();
    let mut sum = 2 * len_row + 2 * len_column - 4;
    let max_from_top_to_bottom = get_treemap_masks_positive(&treemap, false);
    let max_from_left_to_right = get_treemap_masks_positive(&treemap, true);
    let max_from_bottom_to_top = get_treemap_masks_negative(&treemap, false);
    let max_from_right_to_left = get_treemap_masks_negative(&treemap, true);

    for i in 1..len_row - 1 {
        for j in 1..len_column - 1 {
            let cur = treemap[i][j];
            let visible = cur > max_from_top_to_bottom[i - 1][j]
                || cur > max_from_left_to_right[i][j - 1]
                || cur > max_from_bottom_to_top[i + 1][j]
                || cur > max_from_right_to_left[i][j + 1];
            if visible {
                sum += 1;
            }
        }
    }
    println!("{}", sum);
}

pub fn max_visible_trees() {
    let treemap = reshape_treemap_boundary(&get_treemap());
    let treemap_matrix = matrix::from_2d(&treemap);
    let distance_up_matrix = matrix::rot90(
        &matrix::from_2d(&get_least_equal_distances(&matrix::rot90(
            &treemap_matrix,
            1,
        ))),
        3,
    );
    let distance_down_matrix = matrix::flipud(&matrix::rot90(
        &matrix::from_2d(&get_least_equal_distances(&matrix::rot90(
            &matrix::flipud(&treemap_matrix),
            1,
        ))),
        3,
    ));
    let distance_left_matrix = matrix::from_2d(&get_least_equal_distances(&treemap_matrix));
    let distance_right_matrix = matrix::fliplr(&matrix::from_2d(&get_least_equal_distances(
        &matrix::fliplr(&treemap_matrix),
    )));

    let mut max_score = 1;
    for i in 1..treemap_matrix.shape[0] - 1 {
        for j in 1..treemap_matrix.shape[1] - 1 {
            max_score = cmp::max(
                max_score,
                matrix::index(&distance_up_matrix, &[i as usize, j as usize])
                    * matrix::index(&distance_down_matrix, &[i as usize, j as usize])
                    * matrix::index(&distance_left_matrix, &[i as usize, j as usize])
                    * matrix::index(&distance_right_matrix, &[i as usize, j as usize]),
            );
        }
    }
    println!("Max score: {}", max_score);
}
