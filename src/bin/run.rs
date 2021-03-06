macro_rules! run {
    ($day:ident, $part:ident) => {
        let result = aoc2021::$day::$part(include_str!(concat!(
            "../../input/",
            stringify!($day),
            ".txt"
        )));
        println!("{}", result)
    };
}

macro_rules! run_arms {
    ( match ($day:ident, $part:ident) => {  $(pat $pat:pat => $expr:block)*, $( $id:ident)|+ => default }) => {
        match ($day.as_str(), $part.as_str()) {
            $($pat => $expr)*
            $((stringify!($id), "1") => {
                run!($id, part1);
            }
            (stringify!($id), "2") => {
                run!($id, part2);
            })+
            _ => {
                eprintln!("Unknown Day Part combination: Day {} Part {}", $day, $part);
            }
        }
    };
}

pub fn main() {
    let mut args = std::env::args();
    let _bin_name = args.next();
    let day = args.next().unwrap();
    let part = args.next().unwrap();

    run_arms! {
        match (day, part) => {
            pat ("day24", "code") => {
                let code = aoc2021::day24::part1_instructions_to_code(include_str!("../../input/day24.txt"));
                println!("{}", code);
            },
            day1
            | day2
            | day3
            | day4
            | day5
            | day6
            | day7
            | day8
            | day9
            | day10
            | day11
            | day12
            | day13
            | day14
            | day15
            | day16
            | day17
            | day18
            | day19
            | day20
            | day21
            | day22
            | day23
            | day24
            | day25
            => default
        }
    }
}
