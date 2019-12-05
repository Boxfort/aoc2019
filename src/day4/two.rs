fn is_valid(pass: Vec<u8>) -> bool {
    let mut adj = false;
    let mut prev: u8 = 0;
    let mut max: u8 = 0;
    for val in pass {
        if val == prev {
            adj = true;
        }
        if val < max {
            return false;
        }
        if val > prev {
            max = val;
        }
        prev = val;
    }

    adj
}

fn solve() -> usize{
    (372037..905157)
        .step_by(1)
        .filter(|x| is_valid(x.to_string().into_bytes()))
        .count()
}

mod tests {
    use super::solve;

    #[test]
    fn test_day4_part2() {
        let result = solve();
        assert_eq!(result, 481)
    }
}