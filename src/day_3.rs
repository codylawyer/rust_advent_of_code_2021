pub fn part_1(input: &Vec<i32>, num_bits: usize) -> i32 {
    // init gamma and epsilon
    let mut gamma = 0;
    let mut epsilon = 0;

    // init vectors for one and zero counts
    let mut ones = vec![0; num_bits];
    let mut zeros = vec![0; num_bits];
   
    // loop through by bit and see where ones and zeros are
    let base: i32 = 2;
    for bit_idx in 0..num_bits {
        let comp_val = base.pow(bit_idx as u32);
        for idx in 0..input.len() {
            if (input[idx]&comp_val==comp_val) {
                ones[bit_idx] = ones[bit_idx] + 1;
            } else {
                zeros[bit_idx] = zeros[bit_idx] + 1;
            }
        }
    }

    // loop through and if more 1s add to gamma, if more 0s add to epsilon
    for bit_idx in 0..num_bits {
        let val = base.pow(bit_idx as u32);
        if ones[bit_idx] > zeros[bit_idx] {
            gamma += val  
        } else {
            epsilon += val;
        }
    }

    // return gamma*epsilon
    gamma*epsilon
}

//pub fn part_2(input: &Vec<i32>) -> i32 {
//}