use std::io::Write;

fn main() {
    let day_param = if let Some(first_param) = std::env::args().nth(1) {
        first_param
    } else {
        eprint!("Not enough arguments!\nPlease provide number for the day to generate!");
        return;
    };

    let day = if let Ok(day) = day_param.parse::<u8>() {
        day
    } else {
        eprint!("Failed to parse u8!\nPlease provide number for the day to generate!");
        return;
    };

    let mut lib_file = std::fs::OpenOptions::new()
        .append(true)
        .open("src/lib.rs")
        .unwrap();

    let mut mod_file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&format!("src/day{}.rs", day))
        .unwrap();

    let mut bin_part1 = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&format!("src/bin/day{}-1.rs", day))
        .unwrap();

    let mut bin_part2 = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&format!("src/bin/day{}-2.rs", day))
        .unwrap();

    let _example_input = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&format!("input/day{}.example.txt", day))
        .unwrap();

    let _input = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&format!("input/day{}.txt", day))
        .unwrap();

    writeln!(lib_file, "pub mod day{};", day).unwrap();
    write!(
        mod_file,
        "{}",
        include_str!("../../template/lib-mod.rs")
            .replace("dayX", &format!("day{}", day))
            .replace("partX", "part1")
    )
    .unwrap();
    write!(
        bin_part1,
        "{}",
        include_str!("../../template/bin.rs")
            .replace("dayX", &format!("day{}", day))
            .replace("partX", "part1")
    )
    .unwrap();
    write!(
        bin_part2,
        "{}",
        include_str!("../../template/bin.rs")
            .replace("dayX", &format!("day{}", day))
            .replace("partX", "part2")
    )
    .unwrap();
}
