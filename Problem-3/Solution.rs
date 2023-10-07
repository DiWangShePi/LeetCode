use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut hash_map = HashMap::new();
        let len = s.len();
        let mut j = 0;
        let mut length_of_longest = 0;

        for i in 0..len {
            let current_char = s.chars().nth(i).unwrap();

            match hash_map.get(&current_char) {
                Some(&value) => {
                    if length_of_longest < (i - j) {
                        length_of_longest = i - j;
                    }
                    if j <= value {
                        j = value + 1;
                    }
                }
                None => {}
            }
            hash_map.insert(current_char, i);
        }

        if length_of_longest < (len - j) {
            (len - j) as i32
        } else {
            length_of_longest as i32
        }
    }
}