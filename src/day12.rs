use std::collections::HashMap;

pub struct Graph<'a> {
    edges: HashMap<&'a str, Vec<&'a str>>,
}

fn parse_input(input: &str) -> Graph<'_> {
    let edges = input.lines().flat_map(|elem| elem.split_once("-")).fold(
        HashMap::new(),
        |mut map: HashMap<_, Vec<_>>, elem| {
            map.entry(elem.0).or_default().push(elem.1);
            map.entry(elem.1).or_default().push(elem.0);
            map
        },
    );
    Graph { edges }
}

pub enum SmallCaveDuplicateStrategy {
    NoDuplicates,
    AtMostOneDuplicateInTotal,
}

pub fn no_duplicates<'a>(graph: &Graph<'a>, path: &mut Vec<&'a str>) -> u32 {
    let mut unexplored_branches =
        Vec::from([graph.edges.get(path.last().unwrap()).unwrap().as_slice()]);

    let mut count = 0;

    while let Some(branches) = unexplored_branches.pop() {
        match branches {
            [] => {
                path.pop();
            }
            &[head, ref tail @ ..] => {
                unexplored_branches.push(tail);

                if head == "end" {
                    count += 1;
                } else if head.chars().next().unwrap().is_uppercase() || !path.contains(&head) {
                    path.push(head);
                    unexplored_branches.push(graph.edges.get(head).unwrap())
                }
            }
        }
    }

    count
}

pub fn at_most_one_duplicate<'a>(graph: &Graph<'a>, path: &mut Vec<&'a str>) -> u32 {
    let mut unexplored_branches = Vec::from([graph.edges.get("start").unwrap().as_slice()]);

    let mut count = 0;

    while let Some(branches) = unexplored_branches.pop() {
        match branches {
            [] => {
                path.pop();
            }
            &[head, ref tail @ ..] => {
                unexplored_branches.push(tail);

                if head == "end" {
                    count += 1;
                } else if head == "start" {
                } else {
                    let uppercase = head.chars().next().unwrap().is_uppercase();
                    let contained = path.contains(&head);
                    path.push(head);
                    if uppercase || !contained {
                        unexplored_branches.push(graph.edges.get(head).unwrap())
                    } else {
                        count += no_duplicates(graph, path);
                    }
                }
            }
        }
    }

    count
}

pub fn part1(input: &str) -> u32 {
    let graph = parse_input(input);
    no_duplicates(&graph, &mut vec!["start"])
}

pub fn part2(input: &str) -> u32 {
    let graph = parse_input(input);
    at_most_one_duplicate(&graph, &mut vec!["start"])
}

#[test]
fn part1_example1() {
    let input = include_str!(concat!("../input/day12.example1.txt"));
    assert_eq!(part1(input), 10);
}

#[test]
fn part1_example2() {
    let input = include_str!(concat!("../input/day12.example2.txt"));
    assert_eq!(part1(input), 19);
}

#[test]
fn part1_example3() {
    let input = include_str!(concat!("../input/day12.example3.txt"));
    assert_eq!(part1(input), 226);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day12.txt"));
    assert_eq!(part1(input), 4720);
}

#[test]
fn part2_example1() {
    let input = include_str!("../input/day12.example1.txt");
    assert_eq!(part2(input), 36);
}

#[test]
fn part2_example2() {
    let input = include_str!("../input/day12.example2.txt");
    assert_eq!(part2(input), 103);
}

#[test]
fn part2_example3() {
    let input = include_str!("../input/day12.example3.txt");
    assert_eq!(part2(input), 3509);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!("../input/day12.txt"));
    assert_eq!(part2(input), 147848);
}
