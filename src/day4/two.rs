use std::collections::HashMap;

fn is_valid(pass: Vec<u8>) -> bool {
    let mut occurrences: HashMap<u8, u8> = HashMap::new();
    let mut prev: u8 = 10;
    for val in pass {
        occurrences.insert(
            val,
            *occurrences.get(&val).unwrap_or(&0u8) + 1
        );

        if val < prev {
            return false;
        }

        prev = val;
    }

    for val in occurrences {
        if val.1 == 2 {
            return true;
        }
    }

    false
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
        assert_eq!(result, 299)
    }
}