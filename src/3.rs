mod utils;

fn main() {
    let data = utils::fast_get_file_data_as_vec();

    get_power_consumption(&data);
}

fn get_power_consumption(data: &Vec<String>) {
    let mut epsilon_rate = 0;
    let mut gamma_rate = 0;
    for i in 0..data[0].len() {
        epsilon_rate <<= 1;
        gamma_rate <<= 1;
        let mut is_one_common = 0;
        for line in data {
            let b = line.chars().into_iter().collect::<Vec<char>>()[i];
            if b == '1' {
                is_one_common += 1;
            } else {
                is_one_common -= 1;
            }
        }

        epsilon_rate |= (is_one_common > 0) as i32;
        gamma_rate |= (is_one_common < 0) as i32;
    }
    println!("Power consumption: {}", epsilon_rate * gamma_rate);
}

fn get_life_support_rating(data: &Vec<String>) {
    todo!()
}
