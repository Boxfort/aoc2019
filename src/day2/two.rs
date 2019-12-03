fn run_computer(input: &mut Vec<usize>) {
    for i in (0..input.len()).step_by(4) {
        let dest = input[i+3];

        input[dest] = match input[i] {
            // Add
            1 => {
                let x = input[i+1];
                let y = input[i+2];

                input[x] + input[y]
            },
            // Multiply
            2 => {
                let x = input[i+1];
                let y = input[i+2];

                input[x] * input[y]
            },
            // Exit
            99 => {
                break
            },
            _ => panic!("UNEXPECTED INSTRUCTION: {}", input[i])
        };
    }
}

fn solve() -> usize {
    let input: Vec<usize> = super::get_input();

    for noun in 1..100 {
        for verb in 1..100 {
            let mut input_clone = input.clone();
            input_clone[1] = noun;
            input_clone[2] = verb;

            run_computer(&mut input_clone);

            if input_clone[0] == 19690720 {
                return 100 * noun + verb;
            }
        }
    }

    panic!("NO RESULT FOUND.")
}

mod tests {
    use super::solve;

    #[test]
    fn test_day2_part2() {
        let result = solve();
        assert_eq!(result, 8051)
    }
}
