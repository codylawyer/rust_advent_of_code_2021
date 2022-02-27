mod day_1;
mod utils;

use std::env;

fn main() {
    // get day and filename
    let args: Vec<String> = env::args().collect();
    let day = &args[1].parse::<i32>().unwrap();
    let filename = &args[2];

    match day {
        1 => {
                // convert to list of ints
                let input = utils::read_to_ints(&filename);
                println!("{}",day_1::part_1(&input));
                println!("{}",day_1::part_2(&input));
        }
        _ => ()
    }
}
