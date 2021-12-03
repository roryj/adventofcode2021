use adventofcode2021::*;

fn main() {
    println!("Welcome! Advent of Code 2021");

    let depths: Vec<u32> = input_utils::read_all_as("inputs/day1.input");
    println!("Day 1 - Part 1: {}", day1::part1(depths.clone()));
    println!("Day 1 - Part 2: {}", day1::part2(depths));

    let depths: Vec<String> = input_utils::read_all_as("inputs/day2.input");
    println!("Day 2 - Part 1: {}", day2::part1(depths.clone()));
    println!("Day 2 - Part 2: {}", day2::part2(depths));
}
