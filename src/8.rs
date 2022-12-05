use std::collections::HashSet;

mod utils;

fn main() {
    let data = utils::fast_get_file_data_as_vec()
        .iter()
        .map(|l| {
            l.split('|')
                .map(|ds| ds.trim().to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    find_special_digits(&data);
    find_all_digits(&data);
}

/// Finds the number of times 1, 4, 7, 8 appears.
fn find_special_digits(data: &Vec<Vec<String>>) {
    let mut special_digits = [0; 4];
    for line in data {
        let digits = line[1]
            .split_ascii_whitespace()
            .map(String::from)
            .collect::<Vec<String>>();
        for d in digits {
            if d.len() == 2 {
                special_digits[0] += 1;
            } else if d.len() == 4 {
                special_digits[1] += 1;
            } else if d.len() == 3 {
                special_digits[2] += 1;
            } else if d.len() == 7 {
                special_digits[3] += 1;
            }
        }
    }
    println!(
        "Number of special digits: {}",
        special_digits.iter().sum::<i32>()
    );
}

fn find_all_digits(data: &Vec<Vec<String>>) {
    let mut digits: Vec<HashSet<char>> = vec![HashSet::new(); 10];
    let mut total_result = 0;
    for line in data {
        let displays = line[0]
            .split_ascii_whitespace()
            .map(String::from)
            .collect::<Vec<String>>();
        let mut left: HashSet<i32> = HashSet::from_iter(0..10);
        // Find 1, 4, 7, 8.
        for (i, display) in displays.iter().enumerate() {
            let i = i as i32;
            if display.len() == 2 {
                // 1
                digits[1] = HashSet::from_iter(display.chars());
                left.remove(&i);
            } else if display.len() == 4 {
                // 4
                digits[4] = HashSet::from_iter(display.chars());
                left.remove(&i);
            } else if display.len() == 3 {
                // 7
                digits[7] = HashSet::from_iter(display.chars());
                left.remove(&i);
            } else if display.len() == 7 {
                // 8
                digits[8] = HashSet::from_iter(display.chars());
                left.remove(&i);
            }
        }

        // Find 5, 6.
        let mut new_left = left.clone();
        for i in left {
            let display = &displays[i as usize];
            let display_set = HashSet::from_iter(display.chars());
            if display_set.intersection(&digits[4]).count() == 3
                && display_set.intersection(&digits[1]).count() == 1
            {
                if display.len() == 5 {
                    // 5
                    digits[5] = HashSet::from_iter(display.chars());
                    new_left.remove(&i);
                } else if display.len() == 6 {
                    // 6
                    digits[6] = HashSet::from_iter(display.chars());
                    new_left.remove(&i);
                }
            }
        }
        left = new_left;

        // Find 2, 9.
        new_left = left.clone();
        for i in left {
            let display = &displays[i as usize];
            let display_set = HashSet::from_iter(display.chars());
            if display_set.intersection(&digits[1]).count() == 1 {
                // 2
                digits[2] = HashSet::from_iter(display.chars());
                new_left.remove(&i);
            } else if display_set.intersection(&digits[4]).count() == 4 {
                // 9
                digits[9] = HashSet::from_iter(display.chars());
                new_left.remove(&i);
            }
        }
        left = new_left;

        // Find 0, 3.
        for i in left {
            let display = &displays[i as usize];
            if display.len() == 6 {
                // 0
                digits[0] = HashSet::from_iter(display.chars());
            } else if display.len() == 5 {
                // 3
                digits[3] = HashSet::from_iter(display.chars());
            }
        }

        let outputs = line[1]
            .split_ascii_whitespace()
            .map(|o| {
                let o = String::from(o);
                HashSet::from_iter(o.chars())
            })
            .collect::<Vec<HashSet<char>>>();
        let mut result = 0;
        for o in &outputs {
            for (j, digit) in digits.iter().enumerate() {
                if digit.intersection(o).count() == digit.len() && digit.len() == o.len() {
                    result *= 10;
                    result += j;
                    break;
                }
            }
        }
        total_result += result;
    }
    println!("Total output values: {total_result}");
}
