use super::ToPoints;

fn get_intersections(a: &Vec<(i32, i32)>, b: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    // Compare all pairs of points in each line to see if there's an intersection
    for i in 1..(a.len() - 1) {
        for j in 1..(b.len() - 1) {
        }
    }

    return vec!()
}

fn solve() -> i32 {
    // Get input points
    let input = super::get_input();

    // Check all lines to see if they intersect, if so get the point that they intersect at
    let intersections = get_intersections(&input[0].to_points(), &input[1].to_points());

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
