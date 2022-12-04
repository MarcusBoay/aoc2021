mod utils;

fn main() {
    let measurements = utils::fast_get_file_data_as_vec()
        .iter()
        .map(|m| m.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    get_total_larger_than_previous(&measurements);
    get_total_sums_larger_than_previous(&measurements)
}

fn get_total_larger_than_previous(measurements: &[i32]) {
    let mut total = 0;
    for i in 1..measurements.len() {
        if measurements[i - 1] < measurements[i] {
            total += 1;
        }
    }
    println!("Total measurements larger than previous: {total}");
}

fn get_total_sums_larger_than_previous(measurements: &[i32]) {
    let mut total = 0;
    let mut prev_sum: i32 = measurements[0..3].iter().sum();
    for i in 3..measurements.len() {
        let cur_sum = prev_sum - measurements[i - 3] + measurements[i];
        if cur_sum > prev_sum {
            total += 1
        }
        prev_sum = cur_sum;
    }
    println!("Total sums larger than previous: {total}");
}
