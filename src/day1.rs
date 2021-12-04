fn parse_input(input: &str) -> impl Iterator<Item = u32> + '_ {
    input.lines().map(|line| line.parse().unwrap())
}

///
///```rust
/// # use aoc2021::day1::part1;
/// let input = include_str!("../input/day1.example.txt");
///
/// assert_eq!(part1(input), 7);
/// ```
///
pub fn part1(input: &str) -> u32 {
    let mut iter = parse_input(input);

    let mut last = iter.next().unwrap();

    let mut counter = 0;
    for current in iter {
        if current > last {
            counter += 1;
        }
        last = current
    }

    counter
}

///
///```rust
/// # use aoc2021::day1::part2;
/// let input = include_str!("../input/day1.example.txt");
/// assert_eq!(part2(input), 5);
/// ```
///
pub fn part2(input: &str) -> u32 {
    let mut iter = parse_input(input);

    let mut window1 = iter.next().unwrap();
    let mut window2 = iter.next().unwrap();
    let mut window3 = iter.next().unwrap();

    window2 += window3;
    window1 += window2;

    let mut counter = 0;

    for current in iter {
        if window2 + current > window1 {
            counter += 1;
        }
        window1 = window2 + current;
        window2 = window3 + current;
        window3 = current;
    }

    counter
}
