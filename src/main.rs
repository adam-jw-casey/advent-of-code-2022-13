use std::env;
use std::fs;
use advent_of_code_2022_13::sum_correct;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read {file_path}");

    println!("The sum of the indices of packets in correct order is: {}", sum_correct(&contents));
}
