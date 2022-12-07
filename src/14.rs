use std::collections::HashMap;

mod utils;

type PairRuleMap = HashMap<char, HashMap<char, char>>;

struct Polymerizer {
    polymer: Vec<char>,
    pair_rules: PairRuleMap,
    memo_table: HashMap<char, HashMap<char, Vec<HashMap<char, usize>>>>,
    steps: usize,
}

// ASSUMPTION: There will always be a pair insertion rule!
impl Polymerizer {
    pub fn new() -> Self {
        let data = utils::fast_get_file_data_as_vec();

        let polymer = data[0].chars().collect::<Vec<char>>();

        let mut pair_rules: PairRuleMap = HashMap::new();
        for line in data.iter().skip(2) {
            let line = line
                .split(" -> ")
                .map(|s| s.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>();

            pair_rules.entry(line[0][0]).or_insert_with(HashMap::new);
            pair_rules
                .get_mut(&line[0][0])
                .unwrap()
                .entry(line[0][1])
                .or_insert(line[1][0]);
        }

        Self {
            polymer,
            pair_rules,
            memo_table: HashMap::new(),
            steps: 0,
        }
    }

    // Very slow naive method.
    pub fn polymerize(&mut self, steps: usize) {
        self.steps = steps;
        // Get new polymer.
        let mut p = self.polymer.clone();
        for i in 0..steps {
            let mut new_polymer: Vec<char> = vec![];

            for i in 1..p.len() {
                let (l, r) = (p[i - 1], p[i]);
                new_polymer.push(l);
                new_polymer.push(self.pair_rules[&l][&r]);
            }
            new_polymer.push(p[p.len() - 1]);

            p = new_polymer;
            // println!(
            //     "After step {}: {:?}",
            //     i + 1,
            //     p.clone().into_iter().collect::<String>()
            // );
        }

        // Get char frequencies.
        let mut f: HashMap<char, usize> = HashMap::new();
        for c in &p {
            *f.entry(*c).or_insert(0) += 1;
        }
        // println!("Map: {:#?}", f);

        self.print_adjusted_value(&f);
    }

    pub fn fast_polymerization(&mut self, steps: usize) {
        // Reset.
        self.steps = steps;
        self.memo_table = HashMap::new();
        // Idea: backtrack & memoize, dynamic programming.

        // steps-1
        for i in 1..self.polymer.len() {
            self.backtrack(steps - 1, self.polymer[i - 1], self.polymer[i]);
        }
        let mut f: HashMap<char, usize> = HashMap::new();
        for c in &self.polymer {
            *f.entry(*c).or_insert(0) += 1;
        }
        for i in 1..self.polymer.len() {
            let (l, r) = (self.polymer[i - 1], self.polymer[i]);
            for (k, v) in self.memo_table[&l][&r][steps - 1].iter() {
                *f.entry(*k).or_insert(0) += v;
            }
        }

        // println!("Map: {:#?}", f);
        // println!("Memo: {:#?}", self.memo_table);
        self.print_adjusted_value(&f);
    }

    fn backtrack(&mut self, step: usize, l: char, r: char) -> HashMap<char, usize> {
        let c = self.pair_rules[&l][&r];
        // Base case.
        if step == 0 {
            let m = &mut self
                .memo_table
                .entry(l)
                .or_insert_with(HashMap::new)
                .entry(r)
                .or_insert_with(|| vec![HashMap::new(); self.steps])[step];
            m.entry(c).or_insert(1);
            return m.clone();
        }

        // Return previously calculated path.
        if self.memo_table.contains_key(&l)
            && self.memo_table[&l].contains_key(&r)
            && !self.memo_table[&l][&r][step].is_empty()
        {
            return self.memo_table[&l][&r][step].clone();
        }

        let lm = self.backtrack(step - 1, l, c);
        let rm = self.backtrack(step - 1, c, r);
        let m = &mut self
            .memo_table
            .entry(l)
            .or_insert_with(HashMap::new)
            .entry(r)
            .or_insert_with(|| vec![HashMap::new(); self.steps])[step];
        *m.entry(c).or_insert(0) += 1;
        for (k, v) in lm {
            *m.entry(k).or_insert(0) += v;
        }
        for (k, v) in rm {
            *m.entry(k).or_insert(0) += v;
        }

        m.clone()
    }

    fn print_adjusted_value(&self, f: &HashMap<char, usize>) {
        let mut max: usize = 0;
        let mut min: usize = usize::MAX;
        for (_, count) in f {
            min = std::cmp::min(min, *count);
            max = std::cmp::max(max, *count);
        }
        println!("Least: {min}, Most: {max}, Adjusted: {}", max - min);
    }
}

fn main() {
    let mut polymerizer = Polymerizer::new();
    polymerizer.polymerize(10);
    polymerizer.fast_polymerization(10);
    polymerizer.fast_polymerization(40);
}
