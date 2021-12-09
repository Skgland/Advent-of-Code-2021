struct BoardCollector<I>(I);

impl<'a, I> Iterator for BoardCollector<I>
where
    I: Iterator<Item = &'a str>,
{
    type Item = [[u32; 5]; 5];

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()?;
        Some([(); 5].map(|_| {
            let mut line = self
                .0
                .next()
                .unwrap()
                .split(' ')
                .filter(|elem| !elem.is_empty())
                .map(|elem| elem.parse().unwrap());
            [(); 5].map(|_| line.next().unwrap())
        }))
    }
}

fn parse_input(input: &str) -> (Vec<u32>, impl Iterator<Item = [[u32; 5]; 5]> + '_) {
    let mut lines = input.lines();
    let balls = lines
        .next()
        .unwrap()
        .split(',')
        .map(|elem| elem.parse())
        .collect::<Result<_, _>>()
        .unwrap();

    (balls, BoardCollector(lines))
}

pub fn process_board(numbers: &[u32], board: [[u32; 5]; 5]) -> (usize, u32) {
    let mut min = usize::MAX;

    for x in 0..5 {
        let mut max_row = usize::MIN;
        let mut max_column = usize::MIN;

        for y in 0..5 {
            max_column = max_column.max(
                numbers
                    .iter()
                    .position(|&elem| elem == board[y][x])
                    .unwrap(),
            );
            max_row = max_row.max(
                numbers
                    .iter()
                    .position(|&elem| elem == board[x][y])
                    .unwrap(),
            )
        }

        min = min.min(max_row.min(max_column));
    }

    let balls = &numbers[..=min];

    let open_numbers = board
        .iter()
        .flat_map(|elem| elem.iter())
        .filter(|&&elem| !balls.contains(&elem))
        .sum();
    (min, open_numbers)
}

pub enum DesiredResult {
    Win,
    Loose,
}

pub fn both(input: &str, want: DesiredResult) -> u32 {
    let (balls, mut boards) = parse_input(input);

    let (mut rounds, mut remaining_score) = process_board(&balls, boards.next().unwrap());

    for board in boards {
        let (new_rounds, new_remaining_score) = process_board(&balls, board);
        if match want {
            DesiredResult::Win => new_rounds < rounds,
            DesiredResult::Loose => new_rounds > rounds,
        } {
            rounds = new_rounds;
            remaining_score = new_remaining_score;
        }
    }

    balls[rounds] * remaining_score
}

pub fn part1(input: &str) -> u32 {
    both(input, DesiredResult::Win)
}

pub fn part2(input: &str) -> u32 {
    both(input, DesiredResult::Loose)
}

#[test]
fn part1_example() {
    let input = include_str!("../input/day4.example.txt");
    assert_eq!(part1(input), 188 * 24);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day4.txt"));
    assert_eq!(part1(input), 6592);
}

#[test]
fn part2_example() {
    let input = include_str!("../input/day4.example.txt");
    assert_eq!(part2(input), 148 * 13);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!("../input/day4.txt"));
    assert_eq!(part2(input), 31755);
}
