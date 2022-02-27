pub fn part_1(input: &Vec<i32>) -> i32 {
    let mut num_times_increased = 0;

    for idx in 1..input.len() {
        if (input[idx] - input[idx-1]) > 0 {
            num_times_increased += 1;
        }
    }
    num_times_increased
}

pub fn part_2(input: &Vec<i32>) -> i32 {
    let mut num_times_increased = 0;

    for idx in 3..input.len() {
        let window_1 = input[idx-1] + input[idx-2] + input[idx-3];
        let window_2 = input[idx] + input[idx-1] + input[idx-2];
        if (window_2 - window_1) > 0 {
            num_times_increased += 1;
        }
    }
    num_times_increased
}
