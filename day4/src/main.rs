use std::io::BufRead;

fn search(
    prev_row: usize,
    prev_col: usize,
    board: &Vec<Vec<char>>,
    direction: (i32, i32),
    prev: char,
) -> bool {
    let next_ch: char;
    if prev == 'S' {
        return true;
    }
    next_ch = match prev {
        'X' => 'M',
        'M' => 'A',
        'A' => 'S',
        _ => unreachable!(),
    };
    let (row, col): (Result<usize, _>, Result<usize, _>) = (
        (prev_row as i32 + direction.0).try_into(),
        (prev_col as i32 + direction.1).try_into(),
    );
    if let Ok(r) = row {
        if r > board.len() - 1 {
            return false;
        }
    }
    if let Ok(c) = col {
        if c > board[0].len() - 1 {
            return false;
        }
    }

    return match (row, col) {
        (Err(_), _) | (_, Err(_)) => false,
        (Ok(r), Ok(c)) => match board[r][c] {
            x if x == next_ch => return search(r, c, board, direction, next_ch),
            _ => return false,
        },
    };
}

fn main() {
    let board = std::fs::read("real.txt")
        .unwrap()
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut x_positions: Vec<(usize, usize)> = vec![];
    let mut s_positions: Vec<(usize, usize)> = vec![];
    for (i, row) in board.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == 'X' {
                x_positions.push((i, j));
            } else if *col == 'S' {
                s_positions.push((i, j));
            }
        }
    }
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    //println!("{:?}", x_positions);
    let mut count = 0;
    let results = x_positions
        .iter()
        .map(|(row, col)| {
            (
                *row,
                *col,
                directions
                    .iter()
                    .map(|d| {
                        if search(*row, *col, &board, *d, 'X') {
                            1
                        } else {
                            0
                        }
                    })
                    .fold(0, |acc, x| acc + x),
            )
        })
        .collect::<Vec<(usize, usize, u32)>>();

    let x_len = results.iter().fold(0, |acc, (_, _, x)| acc + *x);
    //println!("{:?}", results);
    count += x_len;
    println!("{}", count);
}
