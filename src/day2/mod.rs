use std;

mod one;
mod two;

fn get_input() -> Vec<usize> {
    let mut input: Vec<usize> = vec!();

    let bytes = include_bytes!("input.txt");
    let input_string = std::str::from_utf8(bytes).unwrap();

    for line in input_string.split(",") {
        let num = line.trim().parse::<usize>().unwrap();
        input.push(num);
    }

    input
}
