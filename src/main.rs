use adventofcode2021::*;

fn main() {
    println!("Welcome! Advent of Code 2021");

    let depths: Vec<u32> = input_utils::read_all_as("inputs/day1.input");
    println!("Day 1 - Part 1: {}", day1::part1(depths.clone()));
    println!("Day 1 - Part 2: {}", day1::part2(depths));

    let depths: Vec<String> = input_utils::read_all_as("inputs/day2.input");
    println!("Day 2 - Part 1: {}", day2::part1(depths.clone()));
    println!("Day 2 - Part 2: {}", day2::part2(depths));

    let input: Vec<String> = input_utils::read_all_as("inputs/day3.input");
    println!("Day 3 - Part 1: {}", day3::part1(input.clone()));
    println!("Day 3 - Part 2: {}", day3::part2(input));

    let input: Vec<String> = input_utils::read_all_as("inputs/day4.input");
    println!("Day 4 - Part 1: {}", day4::part1(input.clone()));
    println!("Day 4 - Part 2: {}", day4::part2(input));

    let input: Vec<String> = input_utils::read_all_as("inputs/day5.input");
    println!("Day 5 - Part 1: {}", day5::part1(input.clone()));
    println!("Day 5 - Part 2: {}", day5::part2(input));

    let input: Vec<u8> = input_utils::read_all("inputs/day6.input")
        .first()
        .unwrap()
        .split(",")
        .map(|d| d.parse::<u8>().unwrap())
        .collect();
    println!("Day 6 - Part 1: {}", day6::part1(input.clone(), 80));
    println!("Day 6 - Part 2: {}", day6::part2(input));

    let input: Vec<u32> = input_utils::read_all("inputs/day7.input")
        .first()
        .unwrap()
        .split(",")
        .map(|d| d.parse::<u32>().unwrap())
        .collect();

    println!("Day 7 - Part 1: {}", day7::part1(input.clone()));
    // println!("Day 6 - Part 2: {}", day6::part2(input));
}
