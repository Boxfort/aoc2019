use std;

mod one;
mod two;

fn get_input() -> Vec<i32> {
    let mut input: Vec<i32> = vec!();

    let bytes = include_bytes!("input.txt");
    let input_string = std::str::from_utf8(bytes).unwrap();

    for line in input_string.split("\n") {
        let num = line.parse::<i32>().unwrap();
        input.push(num);
    }

    input
}
