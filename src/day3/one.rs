use super::ToPoints;

fn solve() -> i32 {
    // Get input points
    let input = super::get_input();
    println!("{:?}", input[0].to_points());

    // Check all lines to see if they intersect, if so get the point that they intersect at

    // Take all intersections and get their manhattan distance from (0,0)

    0
}

mod tests {
    use super::solve;

    #[test]
    fn test_day3_part1() {
        let result = solve();
        assert_eq!(result, 4023471)
    }
}
