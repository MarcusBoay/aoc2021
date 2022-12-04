use std::collections::HashSet;

mod utils;

fn main() {
    let data = utils::fast_get_file_data_as_vec();

    let mut draws = data[0]
        .trim()
        .split(',')
        .map(|d| d.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mut boards: Vec<Vec<Vec<i32>>> = vec![];
    let mut i = 0;
    for line in data[2..].iter() {
        if !line.is_empty() {
            let board = line
                .split_ascii_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            if boards.len() <= i / 5 {
                boards.push(vec![]);
            }
            boards[i / 5].push(board);
            i += 1;
        }
    }

    let mut seen_board: Vec<Vec<Vec<i32>>> = vec![];
    for _ in 0..boards.len() {
        let mut cur_seen_board: Vec<Vec<i32>> = vec![];
        for _ in 0..5 {
            cur_seen_board.push(vec![0, 0, 0, 0, 0]);
        }
        seen_board.push(cur_seen_board);
    }

    println!(
        "Final score: {}",
        get_final_score(&draws, boards.clone(), &seen_board)
    );
    println!(
        "Final score: {}",
        get_second_last_board_score(&draws, boards.clone(), &seen_board)
    );
}

fn mark_seen_board(draw: &i32, boards: &Vec<Vec<Vec<i32>>>, seen_board: &mut Vec<Vec<Vec<i32>>>) {
    // Mark draw on board if seen.
    for (i, board) in boards.iter().enumerate() {
        for j in 0..5 {
            for k in 0..5 {
                if *draw == board[j][k] {
                    seen_board[i][j][k] = 1;
                }
            }
        }
    }
}

fn get_final_score(
    draws: &Vec<i32>,
    boards: Vec<Vec<Vec<i32>>>,
    seen_board: &Vec<Vec<Vec<i32>>>,
) -> i32 {
    let mut seen_board = seen_board.clone();

    for draw in draws {
        mark_seen_board(draw, &boards, &mut seen_board);

        // Check if there is a winner
        let mut winning_board = 0;
        let mut has_winner = false;
        for i in 0..boards.len() {
            // Try to find winning row and column.
            for j in 0..5 {
                let mut is_row_winner = true;
                let mut is_col_winner = true;
                for k in 0..5 {
                    is_row_winner &= seen_board[i][j][k] != 0;
                    is_col_winner &= seen_board[i][k][j] != 0;
                }
                if is_row_winner || is_col_winner {
                    winning_board = i;
                    has_winner = true;
                    break;
                }
            }
            if has_winner {
                break;
            }
        }

        if has_winner {
            let i = winning_board;
            let mut final_score = 0;
            for j in 0..5 {
                for k in 0..5 {
                    // Get sum of all unmarked numbers.
                    if seen_board[i][j][k] == 0 {
                        final_score += boards[i][j][k];
                    }
                }
            }
            return final_score * draw;
        }
    }

    unreachable!("There must be an answer.")
}

fn get_second_last_board_score(
    draws: &Vec<i32>,
    boards: Vec<Vec<Vec<i32>>>,
    seen_board: &Vec<Vec<Vec<i32>>>,
) -> i32 {
    let mut seen_board = seen_board.clone();
    let mut wins: HashSet<usize> = HashSet::new();

    for draw in draws {
        mark_seen_board(draw, &boards, &mut seen_board);

        // Check if there is a winner
        let mut winning_board = 0;
        let mut has_winner = false;
        for i in 0..boards.len() {
            // Try to find winning row and column.
            for j in 0..5 {
                let mut is_row_winner = true;
                let mut is_col_winner = true;
                for k in 0..5 {
                    is_row_winner &= seen_board[i][j][k] != 0;
                    is_col_winner &= seen_board[i][k][j] != 0;
                }
                if is_row_winner || is_col_winner {
                    wins.insert(i);
                    winning_board = i;
                    has_winner = wins.len() == boards.len();
                    break;
                }
            }
            if has_winner {
                break;
            }
        }

        if has_winner {
            let i = winning_board;
            let mut final_score = 0;
            for j in 0..5 {
                for k in 0..5 {
                    // Get sum of all unmarked numbers.
                    if seen_board[i][j][k] == 0 {
                        final_score += boards[i][j][k];
                    }
                }
            }
            return final_score * draw;
        }
    }

    unreachable!("There must be an answer.")
}
