use std::num::ParseIntError;
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Debug)]
pub struct CuboidInstruction {
    cuboid: Cuboid,
    state: TargetState,
}

impl FromStr for CuboidInstruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (state, cuboid) = s.split_once(' ').unwrap();
        let state = state.parse()?;
        let cuboid = cuboid.parse()?;

        Ok(CuboidInstruction { state, cuboid })
    }
}

#[derive(Debug)]
pub struct Cuboid {
    x: RangeInclusive<isize>,
    y: RangeInclusive<isize>,
    z: RangeInclusive<isize>,
}

impl Cuboid {
    pub fn size(&self) -> usize {
        self.x.clone().count() * self.y.clone().count() * self.z.clone().count()
    }

    pub fn remove_overlap(self, other: &Self) -> Vec<Self> {
        if let Some(overlap) = self.overlaps(other) {
            self.remove_sub_cuboid(&overlap)
        } else {
            vec![self]
        }
    }

    pub fn overlaps(&self, other: &Self) -> Option<Cuboid> {
        let cuboid = Cuboid {
            x: *self.x.start().max(other.x.start())..=*self.x.end().min(other.x.end()),
            y: *self.y.start().max(other.y.start())..=*self.y.end().min(other.y.end()),
            z: *self.z.start().max(other.z.start())..=*self.z.end().min(other.z.end()),
        };

        Some(cuboid).filter(|cuboid| !cuboid.is_empty())
    }

    fn is_empty(&self) -> bool {
        self.x.is_empty() || self.y.is_empty() || self.z.is_empty()
    }

    fn remove_sub_cuboid(&self, sub: &Self) -> Vec<Cuboid> {
        let pre_x = Cuboid {
            x: *self.x.start()..=(sub.x.start() - 1),
            y: self.y.clone(),
            z: self.z.clone(),
        };
        let post_x = Cuboid {
            x: (sub.x.end() + 1)..=*self.x.end(),
            y: self.y.clone(),
            z: self.z.clone(),
        };
        let remaining_x = Cuboid {
            x: sub.x.clone(),
            y: self.y.clone(),
            z: self.z.clone(),
        };
        let pre_y = Cuboid {
            x: remaining_x.x.clone(),
            y: *remaining_x.y.start()..=sub.y.start() - 1,
            z: remaining_x.z.clone(),
        };
        let post_y = Cuboid {
            x: remaining_x.x.clone(),
            y: sub.y.end() + 1..=*remaining_x.y.end(),
            z: remaining_x.z.clone(),
        };
        let remaining_y = Cuboid {
            x: remaining_x.x,
            y: sub.y.clone(),
            z: remaining_x.z,
        };
        let pre_z = Cuboid {
            x: remaining_y.x.clone(),
            y: remaining_y.y.clone(),
            z: *remaining_y.z.start()..=sub.z.start() - 1,
        };
        let post_z = Cuboid {
            x: remaining_y.x,
            y: remaining_y.y,
            z: sub.z.end() + 1..=*remaining_y.z.end(),
        };

        let mut parts = vec![pre_x, post_x, pre_y, post_y, pre_z, post_z];
        parts.retain(|elem| !elem.is_empty());
        parts
    }
}

impl FromStr for Cuboid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ranges = s
            .splitn(3, ',')
            .into_iter()
            .flat_map(|elem| elem.split_once('='))
            .flat_map(|(_name, range)| range.split_once(".."))
            .map(|(start, end)| Ok((start.parse()?, end.parse()?)))
            .collect::<Result<Vec<(_, _)>, _>>()
            .map_err(|_: ParseIntError| ())?;

        if let [x, y, z] = ranges.as_slice() {
            Ok(Cuboid {
                x: x.0..=x.1,
                y: y.0..=y.1,
                z: z.0..=z.1,
            })
        } else {
            Err(())
        }
    }
}

#[derive(Debug)]
pub enum TargetState {
    On,
    Off,
}

impl FromStr for TargetState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "on" => Ok(Self::On),
            "off" => Ok(Self::Off),
            _ => Err(()),
        }
    }
}

fn parse_input(input: &str) -> impl Iterator<Item = CuboidInstruction> + '_ {
    input.lines().map(|line| line.parse().unwrap())
}

pub fn perform(
    instructions: impl Iterator<Item = CuboidInstruction>,
) -> (Vec<Cuboid>, Vec<Cuboid>) {
    let mut on = vec![];
    let mut off = vec![];

    for CuboidInstruction { cuboid, state } in instructions {
        let (add, remove) = match state {
            TargetState::On => (&mut on, &mut off),
            TargetState::Off => (&mut off, &mut on),
        };

        *remove = std::mem::take(remove)
            .into_iter()
            .flat_map(|elem: Cuboid| elem.remove_overlap(&cuboid))
            .collect();
        add.push(cuboid);
    }

    let [compact_on, compact_off] = [on, off].map(|cuboids: Vec<Cuboid>| {
        cuboids
            .into_iter()
            .fold(vec![], |list: Vec<Cuboid>, cuboid| {
                let mut result: Vec<Cuboid> = list
                    .into_iter()
                    .flat_map(|elem| elem.remove_overlap(&cuboid))
                    .collect();
                result.push(cuboid);
                result
            })
    });

    (compact_on, compact_off)
}

pub fn part1(input: &str) -> usize {
    let (on, _off) = perform(parse_input(input));

    let region = Cuboid {
        x: -50..=50,
        y: -50..=50,
        z: -50..=50,
    };

    on.into_iter()
        .flat_map(|elem| elem.overlaps(&region))
        .map(|elem| elem.size())
        .sum()
}

pub fn part2(input: &str) -> usize {
    let (on, _off) = perform(parse_input(input));

    on.into_iter().map(|elem| elem.size()).sum()
}

#[test]
fn part1_example() {
    let input = include_str!(concat!("../input/day22.example1.txt"));
    assert_eq!(part1(input), 590784);
}

#[test]
fn part1_full() {
    let input = include_str!(concat!("../input/day22.txt"));
    assert_eq!(part1(input), 580012);
}

#[test]
fn part2_example() {
    let input = include_str!("../input/day22.example2.txt");
    assert_eq!(part2(input), 2758514936282235);
}

#[test]
fn part2_full() {
    let input = include_str!(concat!("../input/day22.txt"));
    assert_eq!(part2(input), 1334238660555542);
}
