mod utils;

fn main() {
    let data = utils::fast_get_file_data_as_string()
        .split(',')
        .map(|f| f.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut fish = vec![0_usize; 9];
    for d in data {
        fish[d] += 1;
    }

    get_fish_after_days(fish.clone(), 18);
    get_fish_after_days(fish.clone(), 80);
    get_fish_after_days(fish.clone(), 256);
}

fn get_fish_after_days(mut fish: Vec<usize>, days: i32) {
    for _ in 0..days {
        let mut next_fish = vec![0; 8];
        next_fish.clone_from_slice(&fish[1..]);
        next_fish[6] += fish[0];
        next_fish.push(0);
        next_fish[8] += fish[0];
        fish = next_fish;
    }
    println!(
        "Total fish after {days} days: {}",
        fish.iter().sum::<usize>()
    );
}
