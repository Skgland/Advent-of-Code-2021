type Segments = u8;

#[derive(Debug)]
struct Sequence {
    examples: Vec<Segments>,
    output: [Segments; 4],
}

impl Sequence {
    pub fn decode(&self) -> usize {
        let eight: &Segments = &0b1111111u8;
        let one = self
            .examples
            .iter()
            .find(|&elem| elem.count_ones() == 2)
            .unwrap();
        let four = self
            .examples
            .iter()
            .find(|&elem| elem.count_ones() == 4)
            .unwrap();
        let seven = self
            .examples
            .iter()
            .find(|&elem| elem.count_ones() == 3)
            .unwrap();

        // 0,6,9
        let seg6 = self
            .examples
            .iter()
            .filter(|elem| elem.count_ones() == 6)
            .copied()
            .collect::<Vec<_>>();

        let six = seg6.iter().find(|&elem| elem | one == *eight).unwrap();
        let nine = seg6.iter().find(|&elem| elem | four == *elem).unwrap();
        let zero = seg6
            .iter()
            .find(|&elem| elem != six && elem != nine)
            .unwrap();

        // 2,3,5
        let seg5 = self
            .examples
            .iter()
            .filter(|elem| elem.count_ones() == 5)
            .copied()
            .collect::<Vec<_>>();

        let three = seg5.iter().find(|&elem| elem | one == *elem).unwrap();
        let five = seg5.iter().find(|&elem| elem | six == *six).unwrap();
        let two = seg5
            .iter()
            .find(|&elem| elem != three && elem != five)
            .unwrap();

        let lookup = [zero, one, two, three, four, five, six, seven, eight, nine];
        self.output
            .iter()
            .map(|digit| lookup.iter().position(|&&elem| elem == *digit).unwrap())
            .fold(0, |state, next| state * 10 + next)
    }
}

pub fn str_to_segment(input: &str) -> u8 {
    let in_bytes = input.as_bytes();
    let mut result: u8 = 0;
    for (idx, c) in "abcdefg".as_bytes().iter().enumerate() {
        if in_bytes.contains(c) {
            result |= 1 << idx as u8
        }
    }
    assert_eq!(
        input.len(),
        result.count_ones() as usize,
        "input: {}",
        input
    );
    result
}

fn parse_input(input: &str) -> impl Iterator<Item = Sequence> + '_ {
    input.lines().map(|line| {
        let (examples, outputs) = line.split_once('|').unwrap();
        let examples = examples
            .split(' ')
            .map(str::trim)
            .map(str_to_segment)
            .collect();
        let outputs = outputs
            .split(' ')
            .map(str::trim)
            .filter(|elme| !elme.is_empty())
            .map(str_to_segment)
            .collect::<Vec<_>>();
        let output = [outputs[0], outputs[1], outputs[2], outputs[3]];
        Sequence { examples, output }
    })
}

///
///```rust
/// # use aoc2021::day8::part1;
/// let input = include_str!("../input/day8.example1.txt");
///
/// assert_eq!(part1(input), 0);
/// ```
///
///```rust
/// # use aoc2021::day8::part1;
/// let input = include_str!("../input/day8.example2.txt");
///
/// assert_eq!(part1(input), 26);
/// ```
pub fn part1(input: &str) -> usize {
    parse_input(input)
        .map(|seq| {
            println!("{:?}", seq);
            let res = seq
                .output
                .iter()
                .filter(|seg| matches!(seg.count_ones(), 2 | 3 | 4 | 7))
                .count();
            println!("{}", res);
            res
        })
        .sum()
}

///
///```rust
/// # use aoc2021::day8::part2;
/// let input = include_str!("../input/day8.example1.txt");
/// assert_eq!(part2(input), 5353);
/// ```
///```rust
/// # use aoc2021::day8::part2;
/// let input = include_str!("../input/day8.example2.txt");
/// assert_eq!(part2(input), 61229);
/// ```
///
pub fn part2(input: &str) -> usize {
    parse_input(input).map(|sequence| sequence.decode()).sum()
}
