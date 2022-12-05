mod utils;

#[derive(PartialEq, Debug)]
enum FuelCost {
    Constant,
    Increasing,
}

fn main() {
    let data = utils::fast_get_file_data_as_string()
        .split(',')
        .map(|d| d.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mut positions = vec![0; (data.iter().max().unwrap() + 1) as usize];
    for d in data {
        positions[d as usize] += 1;
    }

    get_min_cost_fuel(positions.clone(), FuelCost::Constant);
    get_min_cost_fuel(positions.clone(), FuelCost::Increasing);
}

fn get_min_cost_fuel(positions: Vec<i32>, fuel_cost: FuelCost) {
    let mut min_cost: Option<i32> = None;
    for i in 0..positions.len() {
        let mut cur_cost = 0;
        for j in 0..positions.len() {
            if fuel_cost == FuelCost::Constant {
                cur_cost += positions[j] * (j as i32 - i as i32).abs();
            } else {
                cur_cost +=
                    positions[j] * (j as i32 - i as i32).abs() * (1 + (j as i32 - i as i32).abs())
                        / 2; // Arithmetic series sum.
            }
        }
        if min_cost.is_none() {
            min_cost = Some(cur_cost);
        } else {
            min_cost = Some(std::cmp::min(cur_cost, min_cost.unwrap()));
        }
    }
    println!(
        "Min cost fuel with {:?} fuel cost: {}",
        fuel_cost,
        min_cost.unwrap()
    );
}
