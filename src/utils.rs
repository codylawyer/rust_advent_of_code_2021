use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::day_2;

pub fn read_to_ints(filename: &str) -> Vec<i32> {
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Initialize vector
    let mut output = Vec::new();

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for line in reader.lines() {
        let line = line.unwrap(); // Ignore errors.
        output.push(line.parse::<i32>().unwrap())
    }

    // return the list of ints
    output
}

pub fn day_2_read(filename: &str) -> Vec<day_2::Command> {
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // Initialize vector
    let mut output = Vec::new();

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for line in reader.lines() {
        let line = line.unwrap(); // Ignore errors.
        if !line.is_empty() {
            let split = line.split(" ");
            let vec: Vec<&str> = split.collect();
            let current_command = day_2::Command {
                direction: vec[0].parse::<day_2::Direction>().unwrap(),
                amount: vec[1].parse::<i32>().unwrap(),
            };
            output.push(current_command)
        }
    }

    // return the vector of commands
    output
}
