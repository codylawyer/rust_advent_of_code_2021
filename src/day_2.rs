enum Direction {
    forward,
    down,
    up,
}

struct Command {
    direction: Direction,
    amount: i32,
}

pub fn part_1(input: &Vec<Command>) -> i32 {
    let mut horizontal_position = 0;
    let mut depth = 0;

    for idx in 0..len(input) {
        let current_command = input[idx];
        let current_direction = current_command
    }

    horizontal_position*depth
}
