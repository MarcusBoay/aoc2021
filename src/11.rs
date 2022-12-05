mod utils;

struct Simulator {
    original_cave: Vec<Vec<u32>>,
    cave: Vec<Vec<u32>>
}

impl Simulator {
    pub fn new() -> Self {
        let cave = utils::fast_get_file_data_as_vec()
            .iter()
            .map(|l| 
                {
                    l.chars().map(|d| d.to_digit(10).unwrap()).collect::<Vec<u32>>()
                }).collect::<Vec<Vec<u32>>>();
        Self {
            original_cave: cave.clone(),
            cave
        }
    }

    pub fn part1(&mut self, steps: usize) {
        self.cave = self.original_cave.clone();
        let mut total_flashes = 0;
        for step in 0..steps {
            // // DEBUG ZONE
            // println!("step: {step}");
            // for line in &self.cave {
            //     println!("{:?}",line);
            // }
            // println!();
            
            self.add_one_to_all();
        
            total_flashes += self.simulate_flash_for_all();
        }
        println!("Total flashes after {steps} steps: {total_flashes}");
    }

    pub fn part2(&mut self) {
        self.cave = self.original_cave.clone();
        let mut i = 0;
        loop {
            i += 1;
            self.add_one_to_all();
            let flashes = self.simulate_flash_for_all();
            if flashes as usize == self.cave.len() * self.cave[0].len() {
                break;
            }
        }
        println!("First step where all octopuses flash: {i}");
    }

    fn add_one_to_all(&mut self) {
        for i in 0..self.cave.len() {
            for j in 0..self.cave[i].len() {
                self.cave[i][j] += 1;
            }
        }
    }

    fn simulate_flash_for_all(&mut self) -> i32 {
        let mut flashes = 0;
            for i in 0..self.cave.len() {
                for j in 0..self.cave[i].len() {
                    flashes += self.simulate_flash(i, j);
                }
            }
        flashes
    }

    fn simulate_flash(&mut self, i: usize, j: usize) -> i32 {
        // Base cases.
        if self.cave[i][j] < 10 {
            return 0;
        }

        // Flash
        self.cave[i][j] = 0;

        // Get list of valid adjacent coords.
        let mut adjacents: Vec<(usize, usize)> = vec![];
        let n = self.cave.len()-1;
        let m = self.cave[i].len()-1;
        if j > 0 { // west
            adjacents.push((i, j-1));
        }
        if i < n && j > 0 { // south-west
            adjacents.push((i+1, j-1));
        }
        if i < n { // south
            adjacents.push((i+1, j));
        }
        if i < n && j < m { // south-east
            adjacents.push((i+1, j+1));
        }
        if j < m { // east
            adjacents.push((i, j+1));
        }
        if i > 0 && j < m { // north-east
            adjacents.push((i-1, j+1));
        }
        if i > 0 { // north
            adjacents.push((i-1, j));
        }
        if i > 0 && j > 0 { // north-west
            adjacents.push((i-1, j-1));
        }

        // Add and try to flash adjacents.
        let mut flashes = 1;
        for (i, j) in adjacents {
            if self.cave[i][j] > 0 {
                self.cave[i][j] += 1;
            }

            flashes += self.simulate_flash(i, j);
        }
        flashes
    }
}

fn main() {
    let mut simulator = Simulator::new();
    simulator.part1(2);
    simulator.part1(10);
    simulator.part1(100);
    simulator.part2();
}