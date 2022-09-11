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
            if input[idx]&val==val {
                ones += 1;
            } else {
                zeros += 1;
            }
        }

        // if more ones, add to gamma
        if ones > zeros {
            gamma += val;
        } else { // if more zeros, add to epsilon
            epsilon += val;
        }
    }

    // return gamma*epsilon
    gamma*epsilon
}

//pub fn part_2(input: &Vec<i32>) -> i32 {
//}