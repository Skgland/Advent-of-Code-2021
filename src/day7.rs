fn parse_input(input: &str) -> impl Iterator<Item = i32> + Clone + '_ {
    input.split(',').map(|elem| elem.parse().unwrap())
}

pub fn part1(input: &str) -> i32 {
    let mut iter: Vec<_> = parse_input(input).collect();
    iter.sort_unstable();
    let median = iter[iter.len() / 2];

    iter.iter().map(|elem| (elem - median).abs()).sum()
}

pub fn part2(input: &str) -> i32 {
    let mut iter: Vec<_> = parse_input(input).collect();
    iter.sort_unstable();

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

#[test]
fn part1_example() {
    let input = include_str!("../input/day7.example.txt");
    assert_eq!(part1(input), 37);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day7.txt"));
    assert_eq!(part1(input), 348996);
}

#[test]
fn part2_example() {
    let input = include_str!("../input/day7.example.txt");
    assert_eq!(part2(input), 168);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!("../input/day7.txt"));
    assert_eq!(part2(input), 98231647);
}
