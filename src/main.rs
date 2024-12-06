mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

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

fn day4() {
    println!(
        "day4[part1: {}, part2: {}]",
        day4::part1().expect("error when calling part 1"),
        day4::part2().expect("error when calling part 2")
    );
}

fn day5() {
    println!(
        "day5[part1: {}, part2: {}]",
        day5::part1().expect("error when calling part 1"),
        day5::part2().expect("error when calling part 2")
    );
}

fn day6() {
    println!(
        "day6[part1: {}, part2: {}]",
        day6::part1().expect("error when calling part 1"),
        day6::part2().expect("error when calling part 2")
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut day_num: u32 = 6;
    if args.len() > 1 {
        day_num = args[1].parse::<u32>().expect("error parsing arg to int!");
    }

    match day_num {
        1 => day1(),
        2 => day2(),
        3 => day3(),
        4 => day4(),
        5 => day5(),
        6 => day6(),
        _ => println!("No day selected!"),
    }
}
