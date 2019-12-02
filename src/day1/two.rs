fn calculate_fuel_recursive(mass: &i32) -> i32 {
    let fuel = (mass / 3) - 2;

    if fuel <= 0 {
        return 0;
    }

    fuel + calculate_fuel_recursive(&fuel)
}

fn solve() -> i32 {
    let input = crate::day1::get_input();

    input.iter().fold(0, |total_fuel, mass| {
        total_fuel + calculate_fuel_recursive(mass)
    })
}

mod tests {
    #[test]
    fn test_day2() {
        let result = crate::day1::two::solve();
        assert_eq!(result, 4726527);
    }
}