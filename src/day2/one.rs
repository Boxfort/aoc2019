fn solve() -> usize {
    let mut input: Vec<usize> = super::get_input();
    input[1] = 12;
    input[2] = 2;

    for i in (0..input.len()).step_by(4) {
        match input[i] {
            // Add
            1 => {
                let x = input[i+1];
                let y = input[i+2];
                let z = input[i+3];

                input[z] = input[x] + input[y];
            },
            // Multiply
            2 => {
                let x = input[i+1];
                let y = input[i+2];
                let z = input[i+3];

                input[z] =  input[x] * input[y];
            },
            // Exit
            99 => {
                break
            },
            _ => panic!("UNEXPECTED INSTRUCTION: {}", input[i])
        };
    }

    return input[0]
}

mod tests {
    use super::solve;

    #[test]
    fn test_day2_part1() {
        let result = solve();
        assert_eq!(result, 4023471)
    }
}
