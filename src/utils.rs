use std::fs::File;
use std::io::{BufRead, BufReader};

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
