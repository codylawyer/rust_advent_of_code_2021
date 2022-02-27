use std::str::FromStr;

pub enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Direction, ()> {
        match s {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err(()),
        }
    }
}

pub struct Command {
    pub direction: Direction,
    pub amount: i32,
}

pub fn part_1(input: &Vec<Command>) -> i32 {
    let mut horizontal_position = 0;
    let mut depth = 0;

    for idx in 0..input.len() {
        let current_command = &input[idx];
        let current_direction = &current_command.direction;
        let current_amount = &current_command.amount;

        match current_direction {
            Direction::Forward 	=> horizontal_position += current_amount,
            Direction::Down 	=> depth += current_amount,
            Direction::Up 		=> depth += -current_amount,
        }
    }

    horizontal_position*depth
}

pub fn part_2(input: &Vec<Command>) -> i32 {
    let mut horizontal_position = 0;
    let mut depth = 0;
    let mut aim = 0;

    for idx in 0..input.len() {
        let current_command = &input[idx];
        let current_direction = &current_command.direction;
        let current_amount = &current_command.amount;

        match current_direction {
            Direction::Forward 	=>  {
                horizontal_position += current_amount;
                depth += aim*current_amount;
            },
            Direction::Down 	=> aim += current_amount,
            Direction::Up 		=> aim += -current_amount,
        }
    }

    horizontal_position*depth
}
