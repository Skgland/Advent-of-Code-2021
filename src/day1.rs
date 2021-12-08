fn parse_input(input: &str) -> impl Iterator<Item = u32> + '_ {
    input.lines().map(|line| line.parse().unwrap())
}

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

#[test]
fn part1_example() {
    let input = include_str!(concat!("../input/day1.example.txt"));
    assert_eq!(part1(input), 7);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day1.txt"));
    assert_eq!(part1(input), 1292);
}

#[test]
fn part2_example() {
    let input = include_str!("../input/day1.example.txt");
    assert_eq!(part2(input), 5);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!("../input/day1.txt"));
    assert_eq!(part2(input), 1262);
}
