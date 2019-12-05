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
    let mut count = 0;

    for x in 372037..905157 {
        if is_valid(x.to_string().into_bytes()) {
            count += 1;
        }
    }

    count
}

mod tests {
    use super::solve;

    #[test]
    fn test_day4_part1() {
        let result = solve();
        assert_eq!(result, 481)
    }
}