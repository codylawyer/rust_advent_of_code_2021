pub fn part_1(input: &Vec<i32>, num_bits: usize) -> i32 {
    // init gamma and epsilon
    let mut gamma = 0;
    let mut epsilon = 0;

    // loop through by bit and see where ones and zeros are
    let base: i32 = 2;
    for bit_idx in 0..num_bits {
        let val = base.pow(bit_idx as u32);
        let mut ones = 0;
        let mut zeros = 0;
        for idx in 0..input.len() {
            if input[idx] & val == val {
                ones += 1;
            } else {
                zeros += 1;
            }
        }

        // if more ones, add to gamma
        if ones > zeros {
            gamma += val;
        } else {
            // if more zeros, add to epsilon
            epsilon += val;
        }
    }

    // return gamma*epsilon
    gamma * epsilon
}

pub fn part_2(input: &Vec<i32>, num_bits: usize) -> i32 {
    // loop through by bit and see where ones and zeros are
    let base: i32 = 2;
    let mut oxygen = 0;
    let mut co2 = 0;

    // oxygen
    let mut current_values = input.clone();
    for bit_idx in (0..num_bits).rev() {
        let val = base.pow(bit_idx as u32);
        if current_values.len() == 1 {
            break;
        }

        let mut ones = 0;
        let mut zeros = 0;
        for idx in 0..current_values.len() {
            if current_values[idx] & val == val {
                ones += 1;
            } else {
                zeros += 1;
            }
        }

        let mut vals_to_remove = Vec::new();
        for idx in (0..current_values.len()).rev() {
            if current_values[idx] & val != val {
                if ones > zeros || zeros == ones {
                    vals_to_remove.push(idx)
                }
            } else {
                if zeros > ones {
                    vals_to_remove.push(idx)
                }
            }
        }
        for idx in 0..vals_to_remove.len() {
            current_values.remove(vals_to_remove[idx]);
        }
        oxygen = current_values[0];
    }

    // c02
    current_values = input.clone();
    for bit_idx in (0..num_bits).rev() {
        if current_values.len() == 1 {
            break;
        }
        let val = base.pow(bit_idx as u32);
        let mut ones = 0;
        let mut zeros = 0;
        for idx in 0..current_values.len() {
            if current_values[idx] & val == val {
                ones += 1;
            } else {
                zeros += 1;
            }
        }

        let mut vals_to_remove = Vec::new();
        for idx in (0..current_values.len()).rev() {
            if current_values[idx] & val != val {
                if zeros > ones {
                    vals_to_remove.push(idx)
                }
            } else {
                if ones > zeros || zeros == ones {
                    vals_to_remove.push(idx)
                }
            }
        }
        for idx in 0..vals_to_remove.len() {
            current_values.remove(vals_to_remove[idx]);
        }
        co2 = current_values[0];
    }
    oxygen * co2
}
