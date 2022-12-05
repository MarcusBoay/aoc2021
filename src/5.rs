mod utils;

type DataType = Vec<((i32, i32), (i32, i32))>;
type DiagramType = Vec<Vec<i32>>;

#[derive(PartialEq)]
enum OverlapType {
    Simple, // Only consider horizontal and vertical lines.
    Full,   // Consider diagonal lines as well.
}

fn main() {
    let data = utils::fast_get_file_data_as_vec();
    let data = data
        .iter()
        .map(|line| {
            let line = line
                .split("->")
                .map(|pair| {
                    let pair = pair
                        .split(',')
                        .map(|x| x.trim().parse::<i32>().unwrap())
                        .collect::<Vec<i32>>();
                    (pair[0], pair[1])
                })
                .collect::<Vec<(i32, i32)>>();
            (line[0], line[1])
        })
        .collect::<DataType>();

    // Make diagram of ocean floor.
    let mut max_x = 0;
    let mut max_y = 0;
    for entry in &data {
        max_x = std::cmp::max(std::cmp::max(max_x, entry.0 .0), entry.1 .0);
        max_y = std::cmp::max(std::cmp::max(max_y, entry.0 .1), entry.1 .1);
    }
    let diagram = vec![vec![0; (max_y + 1) as usize]; (max_x + 1) as usize];

    find_overlaps(&data, diagram.clone(), OverlapType::Simple);
    find_overlaps(&data, diagram.clone(), OverlapType::Full);
}

fn find_overlaps(data: &DataType, mut diagram: DiagramType, overlap_type: OverlapType) {
    for entry in data {
        if entry.0 .0 == entry.1 .0 {
            let i = entry.0 .0;
            let (j_min, j_max) = if entry.0 .1 < entry.1 .1 {
                (entry.0 .1, entry.1 .1)
            } else {
                (entry.1 .1, entry.0 .1)
            };

            let mut j = j_min;
            while j <= j_max {
                diagram[i as usize][j as usize] += 1;
                j += 1;
            }
        } else if entry.0 .1 == entry.1 .1 {
            let j = entry.0 .1;
            let (i_min, i_max) = if entry.0 .0 < entry.1 .0 {
                (entry.0 .0, entry.1 .0)
            } else {
                (entry.1 .0, entry.0 .0)
            };

            let mut i = i_min;
            while i <= i_max {
                diagram[i as usize][j as usize] += 1;
                i += 1;
            }
        } else if overlap_type == OverlapType::Full {
            let i1 = entry.0 .0;
            let i2 = entry.1 .0;
            let j1 = entry.0 .1;
            let j2 = entry.1 .1;

            if i1 < i2 && j1 < j2 {
                let mut i = i1;
                let mut j = j1;
                while i <= i2 && j <= j2 {
                    diagram[i as usize][j as usize] += 1;
                    i += 1;
                    j += 1;
                }
            } else if i1 < i2 && j1 > j2 {
                let mut i = i1;
                let mut j = j1;
                while i <= i2 && j >= j2 {
                    diagram[i as usize][j as usize] += 1;
                    i += 1;
                    j -= 1;
                }
            } else if i1 > i2 && j1 < j2 {
                let mut i = i1;
                let mut j = j1;
                while i >= i2 && j <= j2 {
                    diagram[i as usize][j as usize] += 1;
                    i -= 1;
                    j += 1;
                }
            } else if i1 > i2 && j1 > j2 {
                let mut i = i1;
                let mut j = j1;
                while i >= i2 && j >= j2 {
                    diagram[i as usize][j as usize] += 1;
                    i -= 1;
                    j -= 1;
                }
            }
        }
    }

    let mut overlaps = 0;
    for i in 0..diagram.len() {
        for j in 0..diagram[i].len() {
            if diagram[i][j] > 1 {
                overlaps += 1;
            }
        }
    }
    println!("Total overlaps: {overlaps}");
}
