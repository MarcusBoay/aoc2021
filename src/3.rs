mod utils;

fn main() {
    let data = utils::fast_get_file_data_as_vec();

    get_power_consumption(&data);
    get_life_support_rating(&data);
}

fn get_ones(data: &Vec<String>, i: usize) -> i32 {
    let mut ones = 0;
    for line in data {
        let b = line.chars().into_iter().collect::<Vec<char>>()[i];
        if b == '1' {
            ones += 1;
        } else {
            ones -= 1;
        }
    }
    ones
}

fn get_power_consumption(data: &Vec<String>) {
    let mut epsilon_rate = 0;
    let mut gamma_rate = 0;
    for i in 0..data[0].len() {
        epsilon_rate <<= 1;
        gamma_rate <<= 1;
        let ones = get_ones(data, i);

        epsilon_rate |= (ones > 0) as i32;
        gamma_rate |= (ones < 0) as i32;
    }
    println!("Power consumption: {}", epsilon_rate * gamma_rate);
}

fn get_life_support_rating(data: &Vec<String>) {
    #[derive(PartialEq)]
    enum CommonType {
        Most,
        Least,
    }

    fn get_common(data: &Vec<String>, common_type: CommonType) -> u32 {
        let mut lines = data.clone();
        let mut i = 0;
        loop {
            let mut new_lines: Vec<String> = vec![];
            // Get common element first.
            let ones = get_ones(&lines, i);

            // Get lines with common bit.
            let common_bit = if ones >= 0 {
                if common_type == CommonType::Most {
                    '1'
                } else {
                    '0'
                }
            } else if common_type == CommonType::Most {
                '0'
            } else {
                '1'
            };
            for line in &lines {
                let b = line.chars().into_iter().collect::<Vec<char>>()[i];
                if b == common_bit {
                    new_lines.push(line.clone());
                }
            }

            i = (i + 1) % lines[0].len();
            lines = new_lines;
            if lines.len() <= 1 {
                break;
            }
        }

        // Convert binary to decimal.
        let mut decimal_value = 0;
        for b in lines[0].chars() {
            decimal_value <<= 1;
            decimal_value |= b.to_digit(10).unwrap();
        }
        decimal_value
    }

    let oxygen_generator_rating = get_common(data, CommonType::Most);
    let co2_scrubber_rating = get_common(data, CommonType::Least);

    println!(
        "Life support rating: {}",
        oxygen_generator_rating * co2_scrubber_rating
    );
}
