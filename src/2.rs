mod utils;

enum Direction {
    FORWARD,
    UP,
    DOWN,
}

fn main() {
    let actions = utils::fast_get_file_data_as_vec()
        .iter()
        .map(|x| {
            let x = x.split(' ').map(String::from).collect::<Vec<String>>();
            (
                match x[0].as_str() {
                    "forward" => Direction::FORWARD,
                    "up" => Direction::UP,
                    "down" => Direction::DOWN,
                    _ => panic!("Unexpected action!"),
                },
                x[1].parse::<i32>().unwrap(),
            )
        })
        .collect::<Vec<(Direction, i32)>>();

    get_multiplied_coordinates(&actions);
    get_aimed_coordinates(&actions);
}

fn get_multiplied_coordinates(actions: &[(Direction, i32)]) {
    let mut x_pos = 0;
    let mut y_pos = 0;
    for action in actions {
        match action.0 {
            Direction::FORWARD => x_pos += action.1,
            Direction::DOWN => y_pos += action.1,
            Direction::UP => y_pos -= action.1,
        }
    }
    println!("Multiplied coordinates: {}", x_pos * y_pos);
}

fn get_aimed_coordinates(actions: &[(Direction, i32)]) {
    let mut x_pos = 0;
    let mut aim = 0;
    let mut y_pos = 0;
    for action in actions {
        match action.0 {
            Direction::FORWARD => {
                x_pos += action.1;
                y_pos += aim * action.1;
            }
            Direction::DOWN => aim += action.1,
            Direction::UP => aim -= action.1,
        }
    }
    println!("Horizontal position: {x_pos}, Depth: {y_pos}");
    println!("Multiplied aimed coordinates: {}", x_pos * y_pos);
}
