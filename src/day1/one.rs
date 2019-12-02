fn calculate_fuel(mass: &i32) -> i32 {
    (mass / 3) - 2
}

fn solve() -> i32 {
    let input = super::get_input();

    input.iter().fold(0, |total_fuel, mass| {
        total_fuel + calculate_fuel(mass)
    })
}

mod tests {
    use super::solve;

    #[test]
    fn test_day1_one() {
        let result = solve();
        assert_eq!(result, 3152919);
    }
}
