mod day1;
mod day2;
mod day3;

fn day1() {
    println!(
        "day1[part1: {}, part2: {}]",
        day1::part1().expect("error when calling part 1"),
        day1::part2().expect("error when calling part 2")
    );
}

fn day2() {
    println!(
        "day2[part1: {}, part2: {}]",
        day2::part1().expect("error when calling part 1"),
        day2::part2().expect("error when calling part 2")
    );
}

fn day3() {
    println!(
        "day3[part1: {}, part2: {}]",
        day3::part1().expect("error when calling part 1"),
        day3::part2().expect("error when calling part 2")
    );
}

fn main() {
    // day1();
    // day2();
    day3();
}
