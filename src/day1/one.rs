use std::fs::File;
use std::io::{prelude::BufRead, BufReader};
use std::env;
use std::env::current_dir;

fn calculate_fuel(mass: &i32) -> i32 {
    (mass / 3) - 2
}

fn solve() -> i32 {
    let input = super::get_input();
    let mut total_fuel: i32 = 0;

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
