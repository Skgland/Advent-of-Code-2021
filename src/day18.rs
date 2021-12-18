use std::iter::Sum;
use std::ops::{Add, ControlFlow};
use std::str::FromStr;

#[derive(Eq, PartialEq, Debug, Clone)]
struct SnailNumber {
    left: SnailElement,
    right: SnailElement,
}

impl SnailNumber {
    pub fn max(&self) -> u8 {
        self.left.max().max(self.right.max())
    }

    pub fn depth(&self) -> u8 {
        self.left.depth().max(self.right.depth()) + 1
    }

    pub fn magnitude(&self) -> u32 {
        3 * self.left.magnitude() + 2 * self.right.magnitude()
    }

    pub fn reduce(self) -> Self {
        let mut current = self;
        loop {
            if current.depth() > 4 {
                current = current.explode();
            } else if current.max() >= 10 {
                current = current.split();
            } else {
                break current;
            }
        }
    }

    pub fn explode(mut self) -> Self {
        self.inner_explode(None, None, 1);
        self
    }

    fn inner_explode<'a>(
        &'a mut self,
        mut next_left: Option<&'a mut SnailElement>,
        mut next_right: Option<&'a mut SnailElement>,
        depth: u8,
    ) -> ControlFlow<(), ()> {
        if depth == 4 {
            let (left_value, right_value) = match self {
                SnailNumber {
                    left: SnailElement::Recursion(ref inner),
                    right: _,
                } => {
                    next_right = Some(&mut self.right);
                    match inner.as_ref() {
                        &SnailNumber {
                            left: SnailElement::Literal(left_value),
                            right: SnailElement::Literal(right_value),
                        } => {
                            self.left = SnailElement::Literal(0);
                            (left_value, right_value)
                        }
                        _ => {
                            panic!("Explosion failure nesting too deep!")
                        }
                    }
                }
                SnailNumber {
                    left: _,
                    right: SnailElement::Recursion(inner),
                } => {
                    next_left = Some(&mut self.left);
                    match inner.as_ref() {
                        &SnailNumber {
                            left: SnailElement::Literal(left_value),
                            right: SnailElement::Literal(right_value),
                        } => {
                            self.right = SnailElement::Literal(0);
                            (left_value, right_value)
                        }
                        _ => {
                            panic!("Explosion failure nesting too deep!")
                        }
                    }
                }
                _ => return ControlFlow::Continue(()),
            };
            if let Some(mut left) = next_left {
                loop {
                    match left {
                        SnailElement::Recursion(l) => {
                            // travers to the right most value to our left
                            left = &mut l.right;
                        }
                        SnailElement::Literal(left_literal) => {
                            *left_literal += left_value;
                            break;
                        }
                    }
                }
            }
            if let Some(mut right) = next_right {
                loop {
                    match right {
                        SnailElement::Recursion(r) => {
                            // travers to the left most value to our right
                            right = &mut r.left;
                        }
                        SnailElement::Literal(right_literal) => {
                            *right_literal += right_value;
                            break;
                        }
                    }
                }
            }
            ControlFlow::Break(())
        } else {
            if let SnailElement::Recursion(left) = &mut self.left {
                left.inner_explode(next_left, Some(&mut self.right), depth + 1)?;
            }
            if let SnailElement::Recursion(right) = &mut self.right {
                right.inner_explode(Some(&mut self.left), next_right, depth + 1)?;
            }
            ControlFlow::Continue(())
        }
    }

    pub fn split(mut self) -> Self {
        self.inner_split();
        self
    }

    fn inner_split(&mut self) -> ControlFlow<(), ()> {
        match &mut self.left {
            SnailElement::Literal(x) if *x >= 10 => {
                self.left = SnailElement::Recursion(Box::new(SnailNumber {
                    left: SnailElement::Literal(*x / 2),
                    right: SnailElement::Literal((*x + 1) / 2),
                }));
                return ControlFlow::Break(());
            }
            SnailElement::Literal(_) => {}
            SnailElement::Recursion(inner) => inner.inner_split()?,
        }

        match &mut self.right {
            SnailElement::Literal(x) if *x >= 10 => {
                self.right = SnailElement::Recursion(Box::new(SnailNumber {
                    left: SnailElement::Literal(*x / 2),
                    right: SnailElement::Literal((*x + 1) / 2),
                }));
                ControlFlow::Break(())
            }
            SnailElement::Literal(_) => ControlFlow::Continue(()),
            SnailElement::Recursion(inner) => inner.inner_split(),
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum SnailElement {
    Literal(u8),
    Recursion(Box<SnailNumber>),
}

impl SnailElement {
    pub fn depth(&self) -> u8 {
        match self {
            SnailElement::Literal(_) => 0,
            SnailElement::Recursion(inner) => inner.depth(),
        }
    }

    pub fn max(&self) -> u8 {
        match self {
            SnailElement::Literal(v) => *v,
            SnailElement::Recursion(inner) => inner.max(),
        }
    }

    pub fn magnitude(&self) -> u32 {
        match self {
            SnailElement::Literal(val) => *val as u32,
            SnailElement::Recursion(inner) => inner.magnitude(),
        }
    }
}

impl Add for SnailNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        SnailNumber {
            left: SnailElement::Recursion(Box::new(self)),
            right: SnailElement::Recursion(Box::new(rhs)),
        }
        .reduce()
    }
}

impl Sum<SnailNumber> for Option<SnailNumber> {
    fn sum<I: Iterator<Item = SnailNumber>>(mut iter: I) -> Self {
        let mut current = iter.next()?;
        for s in iter {
            current = current + s;
        }
        Some(current)
    }
}

impl FromStr for SnailNumber {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((result, "")) = parse_number(s) {
            Ok(result)
        } else {
            Err(())
        }
    }
}

fn parse_number(input: &str) -> Option<(SnailNumber, &str)> {
    let input = input.strip_prefix('[')?;
    let (left, input) = parse_element(input)?;
    let input = input.strip_prefix(',')?;
    let (right, input) = parse_element(input)?;
    let remainder = input.strip_prefix(']')?;
    Some((SnailNumber { left, right }, remainder))
}

fn parse_element(input: &str) -> Option<(SnailElement, &str)> {
    match input.as_bytes() {
        [b'[', ..] => {
            let (inner, remainder) = parse_number(input)?;
            Some((SnailElement::Recursion(Box::new(inner)), remainder))
        }
        [c @ (b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9'), r @ ..] => {
            let mut val = *c - b'0';
            let mut chars = 1;
            let mut remainder = r;

            while let [c @ (b'0' | b'1' | b'2' | b'3' | b'4' | b'5' | b'6' | b'7' | b'8' | b'9'), r @ ..] =
                remainder
            {
                remainder = r;
                chars += 1;
                val = val * 10 + (*c - b'0');
            }

            Some((SnailElement::Literal(val), &input[chars..]))
        }
        [c, ..] => {
            println!("Unexpected {}", *c as char);
            None
        }
        _ => None,
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = SnailNumber> + '_ {
    input.lines().map(|line| line.parse().unwrap())
}

pub fn part1(input: &str) -> u32 {
    parse_input(input)
        .sum::<Option<SnailNumber>>()
        .unwrap()
        .magnitude()
}

pub fn part2(input: &str) -> u32 {
    let iter: Vec<_> = parse_input(input).collect();
    let mut max = 0;
    for x in iter.iter() {
        for y in iter.iter() {
            if x != y {
                let a = (x.clone() + y.clone()).magnitude();
                let b = (y.clone() + x.clone()).magnitude();
                max = [max, a, b].into_iter().max().unwrap()
            }
        }
    }
    max
}

#[test]
fn parse_example() {
    let _: SnailNumber = "[1,2]".parse().unwrap();
    let _: SnailNumber = "[[1,2],3]".parse().unwrap();
    let _: SnailNumber = "[9,[8,7]]".parse().unwrap();
    let _: SnailNumber = "[[1,9],[8,5]]".parse().unwrap();
    let _: SnailNumber = "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]".parse().unwrap();
    let _: SnailNumber = "[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]".parse().unwrap();
    let _: SnailNumber = "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]"
        .parse()
        .unwrap();
}

#[test]
fn explode_examples() {
    assert_eq!(
        "[[[[[9,8],1],2],3],4]"
            .parse::<SnailNumber>()
            .unwrap()
            .explode(),
        "[[[[0,9],2],3],4]".parse().unwrap()
    );
    assert_eq!(
        "[7,[6,[5,[4,[3,2]]]]]"
            .parse::<SnailNumber>()
            .unwrap()
            .explode(),
        "[7,[6,[5,[7,0]]]]".parse().unwrap()
    );
    assert_eq!(
        "[[6,[5,[4,[3,2]]]],1]"
            .parse::<SnailNumber>()
            .unwrap()
            .explode(),
        "[[6,[5,[7,0]]],3]".parse().unwrap()
    );
    assert_eq!(
        "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"
            .parse::<SnailNumber>()
            .unwrap()
            .explode(),
        "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".parse().unwrap()
    );
    assert_eq!(
        "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
            .parse::<SnailNumber>()
            .unwrap()
            .explode(),
        "[[3,[2,[8,0]]],[9,[5,[7,0]]]]".parse().unwrap()
    );
}

#[test]
fn split_examples() {
    assert_eq!(
        "[10,0]".parse::<SnailNumber>().unwrap().split(),
        "[[5,5],0]".parse().unwrap()
    );
    assert_eq!(
        "[11,0]".parse::<SnailNumber>().unwrap().split(),
        "[[5,6],0]".parse().unwrap()
    );
    assert_eq!(
        "[12,0]".parse::<SnailNumber>().unwrap().split(),
        "[[6,6],0]".parse().unwrap()
    );
}

#[test]
fn reduce_example() {
    let s1 = "[[[[4,3],4],4],[7,[[8,4],9]]]"
        .parse::<SnailNumber>()
        .unwrap();
    let s2 = "[1,1]".parse::<SnailNumber>().unwrap();
    let r = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
        .parse::<SnailNumber>()
        .unwrap();
    assert_eq!(s1 + s2, r)
}

#[test]
fn sum_example1() {
    let input = include_str!(concat!("../input/day18.example1.txt"));
    assert_eq!(
        parse_input(input).sum::<Option<SnailNumber>>().unwrap(),
        "[[[[1,1],[2,2]],[3,3]],[4,4]]".parse().unwrap()
    )
}

#[test]
fn sum_example2() {
    let input = include_str!(concat!("../input/day18.example2.txt"));
    assert_eq!(
        parse_input(input).sum::<Option<SnailNumber>>().unwrap(),
        "[[[[3,0],[5,3]],[4,4]],[5,5]]".parse().unwrap()
    )
}

#[test]
fn sum_example3() {
    let input = include_str!(concat!("../input/day18.example3.txt"));
    assert_eq!(
        parse_input(input).sum::<Option<SnailNumber>>().unwrap(),
        "[[[[5,0],[7,4]],[5,5]],[6,6]]".parse().unwrap()
    )
}

#[test]
fn sum_example4() {
    let input = include_str!(concat!("../input/day18.example4.txt"));
    assert_eq!(
        parse_input(input).sum::<Option<SnailNumber>>().unwrap(),
        "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
            .parse()
            .unwrap()
    )
}

#[test]
fn sum_example5() {
    let input = include_str!(concat!("../input/day18.example5.txt"));
    assert_eq!(
        parse_input(input).sum::<Option<SnailNumber>>().unwrap(),
        "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"
            .parse()
            .unwrap()
    )
}

#[test]
fn magnitude_examples() {
    assert_eq!("[9,1]".parse::<SnailNumber>().unwrap().magnitude(), 29);
    assert_eq!("[1,9]".parse::<SnailNumber>().unwrap().magnitude(), 21);
    assert_eq!(
        "[[9,1],[1,9]]".parse::<SnailNumber>().unwrap().magnitude(),
        129
    );
    assert_eq!(
        "[[1,2],[[3,4],5]]"
            .parse::<SnailNumber>()
            .unwrap()
            .magnitude(),
        143
    );
    assert_eq!(
        "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
            .parse::<SnailNumber>()
            .unwrap()
            .magnitude(),
        1384
    );
    assert_eq!(
        "[[[[1,1],[2,2]],[3,3]],[4,4]]"
            .parse::<SnailNumber>()
            .unwrap()
            .magnitude(),
        445
    );
    assert_eq!(
        "[[[[3,0],[5,3]],[4,4]],[5,5]]"
            .parse::<SnailNumber>()
            .unwrap()
            .magnitude(),
        791
    );
    assert_eq!(
        "[[[[5,0],[7,4]],[5,5]],[6,6]]"
            .parse::<SnailNumber>()
            .unwrap()
            .magnitude(),
        1137
    );
    assert_eq!(
        "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
            .parse::<SnailNumber>()
            .unwrap()
            .magnitude(),
        3488
    );
}

#[test]
fn part1_example() {
    let input = include_str!(concat!("../input/day18.example5.txt"));
    assert_eq!(part1(input), 4140);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day18.txt"));
    assert_eq!(part1(input), 4124);
}

#[test]
fn part2_example() {
    let input = include_str!("../input/day18.example5.txt");
    assert_eq!(part2(input), 3993);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!("../input/day18.txt"));
    assert_eq!(part2(input), 4673);
}
