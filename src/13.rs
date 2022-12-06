use std::{collections::HashSet, borrow::BorrowMut};

mod utils;

fn main() {
    let data = utils::fast_get_file_data_as_vec();

    // Get points.
    let mut points: HashSet<(usize, usize)> = HashSet::new(); // (x, y)
    let mut n = 0;
    for line in &data {
        if line.is_empty() {
            // Input for fold instruction lines.
            n += 1;
            break;
        }
        let coords = line.split(',').map(|c| c.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        points.insert((coords[0], coords[1]));
        n += 1;
    }

    // Get fold lines.
    let mut fold_lines: Vec<(usize, usize)> = vec![];
    for i in n..data.len() {
        let (c, v) = (data[i].chars().collect::<Vec<char>>()[11], data[i][13..].to_string().parse::<usize>().unwrap());
        if c == 'x' {
            fold_lines.push((v, 0));
        } else {
            fold_lines.push((0, v));
        }
    }

    fold(points, &fold_lines);
}

fn fold(mut points: HashSet<(usize, usize)>, fold_lines: &Vec<(usize, usize)>) {
    for (i, (fx, fy)) in fold_lines.iter().enumerate() {
        let mut new_points: HashSet<(usize, usize)> = HashSet::new();
        for (x, y) in points.iter() {
            if fx > &0 && x > fx {
                let x = 2*fx - x;
                new_points.insert((x, *y));
            } else if fy > &0 && y > fy {
                let y = 2*fy - y;
                new_points.insert((*x, y));
            } else if (fx > &0 && x < fx) || (fy > &0 && y < fy) {
                new_points.insert((*x, *y));
            }
        }
        println!("Dots after {i} fold along ({fx},{fy}): {}", new_points.len());
        points = new_points;
    }
    print_points(&points);
}

fn print_points(points: &HashSet<(usize, usize)>) {
    let mut max_x = 0;
    let mut max_y = 0;
    for (x, y) in points.iter() {
        max_x = std::cmp::max(*x, max_x);
        max_y = std::cmp::max(*y, max_y);
    }
    
    for y in 0..max_y+1 {
        for x in 0..max_x+1 {
            if points.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}