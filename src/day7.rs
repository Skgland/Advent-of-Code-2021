fn parse_input(input: &str) -> impl Iterator<Item = i32> + Clone + '_ {
    input.split(',').map(|elem| elem.parse().unwrap())
}

///
///```rust
/// # use aoc2021::day7::part1;
/// let input = include_str!("../input/day7.example.txt");
///
/// assert_eq!(part1(input), 37);
/// ```
///
pub fn part1(input: &str) -> i32 {
    let mut iter: Vec<_> = parse_input(input).collect();
    iter.sort();
    let median = iter[iter.len() / 2];

    iter.iter().map(|elem| (elem - median).abs()).sum()
}

///
///```rust
/// # use aoc2021::day7::part2;
/// let input = include_str!("../input/day7.example.txt");
/// assert_eq!(part2(input), 168);
/// ```
///
pub fn part2(input: &str) -> i32 {
    let mut iter: Vec<_> = parse_input(input).collect();
    iter.sort();

    let min = iter.first().unwrap();
    let max = iter.last().unwrap();

    fn cost(dist: i32) -> i32 {
        dist * (dist + 1) / 2
    }

    // TODO can we do better than enumerating all values between min and max?

    (*min..=*max)
        .map(|dest| iter.iter().map(|start| cost((start - dest).abs())).sum())
        .min()
        .unwrap()
}
