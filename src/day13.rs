use std::collections::HashSet;

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub struct Dot {
    x: u16,
    y: u16,
}

#[derive(Debug, Copy, Clone)]
pub enum Fold {
    X(u16),
    Y(u16),
}

struct Input {
    dots: HashSet<Dot>,
    folds: Vec<Fold>,
}

fn parse_input(input: &str) -> Input {
    let mut lines = input.lines();
    let dots = (&mut lines)
        .take_while(|line| !line.trim().is_empty())
        .flat_map(|line| line.split_once(','))
        .map(|(x, y)| Dot {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        })
        .collect();

    let folds: Vec<_> = lines.flat_map(|line| line.strip_prefix("fold along ")).flat_map(|fold|fold.split_once("=")).map(|fold|match fold.0 {
        "x" => Fold::X,
        "y"=> Fold::Y,
        _ => panic!("invalid fold")
    }(fold.1.parse().unwrap())).collect();
    Input { dots, folds }
}

pub fn apply_fold(dots: &mut HashSet<Dot>, fold: Fold) {
    match fold {
        Fold::X(x) => {
            let to_map = dots
                .iter()
                .filter(|dot| dot.x > x)
                .copied()
                .collect::<Vec<_>>();
            dots.retain(|dot| dot.x < x);
            for dot in to_map {
                dots.insert(Dot {
                    x: 2 * x - dot.x,
                    y: dot.y,
                });
            }
        }
        Fold::Y(y) => {
            let to_map = dots
                .iter()
                .filter(|dot| dot.y > y)
                .copied()
                .collect::<Vec<_>>();
            dots.retain(|dot| dot.y < y);
            for dot in to_map {
                dots.insert(Dot {
                    x: dot.x,
                    y: 2 * y - dot.y,
                });
            }
        }
    }
}

pub fn part1(input: &str) -> usize {
    let mut input = parse_input(input);
    apply_fold(&mut input.dots, *input.folds.first().unwrap());
    input.dots.len()
}

pub fn part2(input: &str) -> String {
    let mut input = parse_input(input);
    for fold in input.folds.into_iter() {
        apply_fold(&mut input.dots, fold);
    }
    let max_x = input.dots.iter().map(|elem| elem.x).max().unwrap();
    let max_y = input.dots.iter().map(|elem| elem.y).max().unwrap();

    let mut result = String::new();
    for y in 0..=max_y {
        for x in 0..=max_x {
            if input.dots.contains(&Dot { x, y }) {
                result += "#";
            } else {
                result += ".";
            }
        }
        result += "\n";
    }
    result
}

#[test]
fn part1_example() {
    let input = include_str!(concat!("../input/day13.example.txt"));
    assert_eq!(part1(input), 17);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day13.txt"));
    assert_eq!(part1(input), 790);
}

#[test]
fn part2_example() {
    let input = include_str!("../input/day13.example.txt");
    // SQUARE
    assert_eq!(
        part2(input),
        String::from(
            "\
#####
#...#
#...#
#...#
#####
\
"
        )
    );
}

#[test]
fn part2_full() {
    let input = include_str!(concat!("../input/day13.txt"));
    // PGHZBFJC
    assert_eq!(
        part2(input),
        String::from(
            "\
###...##..#..#.####.###..####...##..##.
#..#.#..#.#..#....#.#..#.#.......#.#..#
#..#.#....####...#..###..###.....#.#...
###..#.##.#..#..#...#..#.#.......#.#...
#....#..#.#..#.#....#..#.#....#..#.#..#
#.....###.#..#.####.###..#.....##...##.
\
"
        )
    );
}
