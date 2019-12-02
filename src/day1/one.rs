use std::fs::File;
use std::io::{prelude::BufRead, BufReader};
use std::env;
use std::env::current_dir;

fn get_input() -> Vec<u32> {
    let mut input: Vec<u32> = vec!();

    let bytes = include_bytes!("input.txt");
    let input_string = std::str::from_utf8(bytes).unwrap();

    for line in input_string.split("\n") {
        let num = line.parse::<u32>().unwrap();
        input.push(num);
    }

    input
}

fn calculate_fuel(mass: &u32) -> u32 {
    (mass / 3) - 2
}

fn solve() -> u32 {
    let input = get_input();
    let mut total_fuel: u32 = 0;

    input.iter().for_each(|mass| {
        total_fuel += calculate_fuel(mass);
    });

    total_fuel
}

mod tests {
    #[test]
    fn test_day1() {
        let result = crate::day1::one::solve();
        assert_eq!(result, 3152919);
    }
}
