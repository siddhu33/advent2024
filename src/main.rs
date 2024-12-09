mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

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

fn day7() {
    println!(
        "day7[part1: {}, part2: {}]",
        day7::part1().expect("error when calling part 1"),
        day7::part2().expect("error when calling part 2")
    );
}

fn day8() {
    println!(
        "day8[part1: {}, part2: {}]",
        day8::part1().expect("error when calling part 1"),
        day8::part2().expect("error when calling part 2")
    );
}

fn day9() {
    println!(
        "day9[part1: {}, part2: {}]",
        day9::part1().expect("error when calling part 1"),
        day9::part2().expect("error when calling part 2")
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut day_num: u32 = 9;
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
        7 => day7(),
        8 => day8(),
        9 => day9(),
        _ => println!("No day selected!"),
    }
}
