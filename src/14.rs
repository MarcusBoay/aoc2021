use std::{collections::HashMap, i32::MAX};

mod utils;

type PairRuleMap = HashMap<char, HashMap<char, char>>;

fn main() {
    let data = utils::fast_get_file_data_as_vec();

    let polymer = data[0].chars().collect::<Vec<char>>();

    let mut pi: PairRuleMap = HashMap::new();
    for line in data.iter().skip(2) {
        let line = line
            .split(" -> ")
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        pi.entry(line[0][0]).or_insert_with(HashMap::new);
        pi.get_mut(&line[0][0])
            .unwrap()
            .entry(line[0][1])
            .or_insert(line[1][0]);
    }

    polymerize(polymer, 10, &pi);
}

fn polymerize(mut polymer: Vec<char>, steps: usize, pi: &PairRuleMap) {
    // Get new polymer.
    for i in 0..steps {
        let mut new_polymer: Vec<char> = vec![];

        for i in 1..polymer.len() {
            let (l, r) = (polymer[i - 1], polymer[i]);
            // ASSUMPTION: There will always be a pair insertion rule!
            // let did_insert = if let Some(cm) = pi.get_key_value(&l) {
            //     if let Some(c) = cm.1.get_key_value(&r) {
            //         return true;
            //     } else {
            //         return false;
            //     }
            // };
            // pi.get_key_value(&l).is
            new_polymer.push(l);
            new_polymer.push(pi[&l][&r]);
        }
        new_polymer.push(polymer[polymer.len() - 1]);

        polymer = new_polymer;
        // println!(
        //     "After step {}: {:?}",
        //     i + 1,
        //     polymer.clone().into_iter().collect::<String>()
        // );
    }

    // Get char frequencies.
    let mut f: HashMap<char, i32> = HashMap::new();
    for c in &polymer {
        *f.entry(*c).or_insert(0) += 1;
    }

    let mut max = 0;
    let mut min = MAX;
    for (_, count) in f {
        min = std::cmp::min(min, count);
        max = std::cmp::max(max, count);
    }
    println!("Least: {min}, Most: {max}, Adjusted: {}", max - min);
}

fn calculate_polymerization(mut polymer: Vec<char>, steps: usize, pi: &PairRuleMap) {
    // ASSUMPTION: There will always be a pair insertion rule!
}
