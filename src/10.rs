use std::collections::HashMap;

mod utils;

fn main() {
    let nav_map = utils::fast_get_file_data_as_vec();

    let (illegal_chars, incomplete_lines) = separate_lines(&nav_map);
    sum_illegal_chars(&illegal_chars);
    complete_lines(incomplete_lines);
}

/// Gets illegal chars for illegal lines and remaining chars for the incomplete lines.
fn separate_lines(nav_map: &Vec<String>) -> (Vec<char>, Vec<Vec<char>>) {
    let mut first_illegal_chars: Vec<char> = vec![];
    let mut incomplete_lines: Vec<Vec<char>> = vec![];
    for nav_line in nav_map {
        let mut stack: Vec<char> = vec![];
        let mut is_illegal = false;
        for c in nav_line.chars() {
            if stack.len() > 0 {
                let o = *stack.last().unwrap();
                if (o == '(' && c == ')') || (o == '[' && c == ']') || (o == '{' && c == '}') || (o == '<' && c == '>')
                {
                    stack.pop();
                } else if vec!['(', '{', '[', '<'].contains(&c) {
                    stack.push(c);
                } else {
                    first_illegal_chars.push(c);
                    is_illegal = true;
                    break;
                }
            } else {
                stack.push(c);
            }
        }
        if !is_illegal {
            incomplete_lines.push(stack);
        }
    }

    (first_illegal_chars, incomplete_lines)
}

fn sum_illegal_chars(illegal_chars: &Vec<char>) {
    let illegal_map = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137),
    ]);
    let total_sum: i32 = illegal_chars.iter().map(|c| 
            return *illegal_map.get(c).unwrap()
    ).collect::<Vec<i32>>().iter().sum::<i32>();
    println!("Total sum of syntax errors: {total_sum}");
}

fn complete_lines(incomplete_lines: Vec<Vec<char>>) {
    let mut scores: Vec<i64> = vec![];
    for mut line in incomplete_lines {
        let mut cur_score: i64 = 0;

        while line.len() > 0 {
            let c = line.pop().unwrap();
            cur_score *= 5;
            cur_score += match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => panic!("unexpected char")
            };
        }
        scores.push(cur_score);
    }

    scores.sort();
    println!("Middle score: {}", scores[scores.len()/2]);
}