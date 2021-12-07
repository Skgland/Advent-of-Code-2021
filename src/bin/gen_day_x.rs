use std::io::Write;

fn main() {
    let day = std::env::args().nth(1).unwrap().parse::<u8>().unwrap();

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