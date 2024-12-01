mod day1;

fn day1() {
    println!(
        "day1[part1: {}, part2: {}]",
        day1::part1().expect("error when calling part 1"),
        day1::part2().expect("error when calling part 2")
    );
}

fn main() {
    day1();
}
