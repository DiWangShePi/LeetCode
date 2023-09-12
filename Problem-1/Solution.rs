use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut my_map = HashMap::new();

        let len = nums.len();
        let mut current_num = 0;

        for i in 0..len {
            current_num = nums[i];

            match my_map.get(&current_num) {
                Some(&value) => return vec![value, i as i32],
                None => {
                    my_map.insert(target - current_num, i as i32);
                }
            }
        }
        
        Vec::new()
    }
}

fn main() {
    
}

