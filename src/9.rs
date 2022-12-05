use std::collections::VecDeque;

mod utils;

fn main() {
    let map = utils::fast_get_file_data_as_vec().iter().map(|l| {
        l.chars().map(|n| {
            n.to_digit(10).unwrap()
        }).collect::<Vec<u32>>()
    }).collect::<Vec<Vec<u32>>>();

    let low_points = find_low_points(&map);
    sum_low_points(&low_points, &map);
    get_basin_sizes(&low_points, &map);
}

/// Returns low points in the map as coordinates (i, j).
fn find_low_points(map: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut low_points: Vec<(usize, usize)> = vec![];
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if !((i > 0 && map[i-1][j] <= map[i][j]) // up is lower
            || (i < map.len()-1 && map[i+1][j] <= map[i][j]) // down is lower
            || (j > 0 && map[i][j-1] <= map[i][j]) // left is lower
            || (j < map[i].len()-1 && map[i][j+1] <= map[i][j])) // right is lower
            {
                low_points.push((i, j));
            }
        }
    }
    low_points
}

/// Sums low points in map.
fn sum_low_points(low_points: &Vec<(usize, usize)>, map: &Vec<Vec<u32>>) {
    let mut low_points_sum = 0;
    for (i, j) in low_points {
        low_points_sum += map[*i][*j]+1;
    }
    println!("Low points sum: {low_points_sum}");
}

/// Gets sizes of all basins.
fn get_basin_sizes(low_points: &Vec<(usize, usize)>, map: &Vec<Vec<u32>>) {
    let mut seen_map = vec![vec![false; map[0].len()]; map.len()];
    let mut basin_sizes: Vec<i32> = vec![];
    for (li, lj) in low_points {
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        queue.push_back((*li, *lj));
        let mut cur_basin_size = 0;
        while queue.len() > 0 {
            let (i, j) = queue.pop_front().unwrap();
            seen_map[i][j] = true;
            cur_basin_size += 1;

            if i > 0 && !seen_map[i-1][j] && map[i-1][j] < 9 { // add up.
                queue.push_back((i-1, j));
                seen_map[i-1][j] = true;
            }
            if i < map.len()-1 && !seen_map[i+1][j] && map[i+1][j] < 9 { // add down.
                queue.push_back((i+1, j));
                seen_map[i+1][j] = true;
            }
            if j > 0 && !seen_map[i][j-1] && map[i][j-1] < 9 { // add left.
                queue.push_back((i, j-1));
                seen_map[i][j-1] = true;
            }
            if j < map[i].len()-1 && !seen_map[i][j+1] && map[i][j+1] < 9 { // add right.
                queue.push_back((i, j+1));
                seen_map[i][j+1] = true;
            }
        }
        basin_sizes.push(cur_basin_size);
    }

    println!("Basin sizes: {:?}", basin_sizes);
    basin_sizes.sort();
    basin_sizes.reverse();
    let mut result = 1;
    for i in 0..3 {
        result *= basin_sizes[i];
    }
    println!("Result: {result}");
}