fn calculate_fuel(mass: &i32) -> i32 {
    (mass / 3) - 2
}

fn solve() -> i32 {
    let input = crate::day1::get_input();

    input.iter().fold(0, |total_fuel, mass| {
        total_fuel + calculate_fuel(mass)
    })
}

mod tests {
    #[test]
    fn test_day1() {
        let result = crate::day1::one::solve();
        assert_eq!(result, 3152919);
    }
}
