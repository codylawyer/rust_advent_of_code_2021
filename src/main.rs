mod day_1;
mod day_2;
mod day_3;
mod utils;

use std::env;

fn main() {
    // get day and filename
    let args: Vec<String> = env::args().collect();
    let day = &args[1].parse::<i32>().unwrap();
    let filename = &args[2];

    match day {
        1 => {
            // convert to vector of ints
            let input = utils::read_to_ints(&filename);
            println!("{}", day_1::part_1(&input));
            println!("{}", day_1::part_2(&input));
        }
        2 => {
            // convert to vector of commands
            let input = utils::day_2_read(&filename);
            println!("{}", day_2::part_1(&input));
            println!("{}", day_2::part_2(&input));
        }
        3 => {
            // convert to vector of strings
            let (input, num_bits) = utils::day_3_read(&filename);
            println!("{:?}", day_3::part_1(&input, num_bits));
            println!("{:?}", day_3::part_2(&input, num_bits));
        }
        _ => (),
    }
}
