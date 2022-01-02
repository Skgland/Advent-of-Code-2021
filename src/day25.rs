use crate::day25::Spot::{Down, Empty, Right};

enum Spot {
    Down,
    Right,
    Empty,
}

fn parse_input(input: &str) -> Vec<Vec<Spot>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    'v' => Down,
                    '>' => Right,
                    '.' => Empty,
                    _ => panic!(),
                })
                .collect()
        })
        .collect()
}

pub fn part1(input: &str) -> u32 {
    let mut game_board = parse_input(input);

    let mut empty = vec![];

    for (row_idx, row) in game_board.iter().enumerate() {
        for (column_idx, entry) in row.iter().enumerate() {
            match entry {
                Empty => {
                    empty.push((row_idx, column_idx));
                }
                Right | Down => {}
            }
        }
    }

    let mut empty_down = vec![];
    let mut empty_right = empty;
    let mut not_moved;
    let mut iterations = 0;

    loop {
        not_moved = true;

        empty_right
            .drain(..)
            .flat_map(|(row, column)| {
                let origin = (
                    row,
                    (column + game_board[row].len() - 1) % game_board[row].len(),
                );
                if matches! { game_board[origin.0][origin.1], Right } {
                    empty_down.push(origin);
                    not_moved = false;
                    Some((origin, (row, column)))
                } else {
                    empty_down.push((row, column));
                    None
                }
            })
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|(origin, (row, column))| {
                game_board[origin.0][origin.1] = Empty;
                game_board[row][column] = Right;
            });

        empty_down
            .drain(..)
            .flat_map(|(row, column)| {
                let origin = ((row + game_board.len() - 1) % game_board.len(), column);
                if matches! { game_board[origin.0][origin.1], Down } {
                    empty_right.push(origin);
                    not_moved = false;
                    Some((origin, (row, column)))
                } else {
                    empty_right.push((row, column));
                    None
                }
            })
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|(origin, (row, column))| {
                game_board[origin.0][origin.1] = Empty;
                game_board[row][column] = Down;
            });

        iterations += 1;

        if not_moved {
            break;
        }
    }

    iterations
}

pub fn part2(_input: &str) -> u32 {
    println!("There is no part 2!");
    0
}

#[test]
fn part1_example() {
    let input = include_str!(concat!("../input/day25.example.txt"));
    assert_eq!(part1(input), 58);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day25.txt"));
    assert_eq!(part1(input), 509);
}
