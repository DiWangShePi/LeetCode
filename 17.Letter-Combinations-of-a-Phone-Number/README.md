# 17. Letter Combinations of a Phone Number

### 题目描述

给定一个包含数字2-9的字符串，返回该数字可能表示的所有可能的字母组合。以任意顺序返回答案。
下面给出了数字到字母的映射。注意，1不能映射到任何字母。

![](./1200px-telephone-keypad2svg.png)

### 题目解析

采用回溯算法

### 代码实现

###### c++

```c++
#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    vector<string> letterCombinations(string digits) {
        if (digits.empty()) {
            return {};
        }

        unordered_map<char, string> dict = {
            {'2', "abc"}, {'3', "def"}, {'4', "ghi"},
            {'5', "jkl"}, {'6', "mno"}, {'7', "pqrs"},
            {'8', "tuv"}, {'9', "wxyz"}
        };

        vector<string> combinations;
        backtrack(0, "", digits, dict, combinations);
        return combinations;
    }

private:
    void backtrack(int index, string path, const string& digits, const unordered_map<char, string>& letters, vector<string>& combinations) {
        if (path.size() == digits.size()) {
            combinations.push_back(path);
            return;
        }

        string possibleLetters = letters.at(digits[index]);
        for (char letter : possibleLetters) {
            backtrack(index + 1, path + letter, digits, letters, combinations);
        }
    }
};
```

###### rust

```rust
use std::collections::HashMap;

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        if digits.is_empty() {
            return Vec::new();
        }

        let dict: HashMap<char, String> = vec![
            ('2', "abc".to_string()),
            ('3', "def".to_string()),
            ('4', "ghi".to_string()),
            ('5', "jkl".to_string()),
            ('6', "mno".to_string()),
            ('7', "pqrs".to_string()),
            ('8', "tuv".to_string()),
            ('9', "wxyz".to_string()),
        ].into_iter().collect();

        let mut combinations: Vec<String> = Vec::new();
        Self::back_track(0, String::new(), &digits, &dict, &mut combinations);
        combinations
    }

    fn back_track(index: usize, path: String, digits: &str, dict: &HashMap<char, String>, com: &mut Vec<String>) {
        if path.len() == digits.len() {
            com.push(path);
            return;
        }

        if let Some(digit) = digits.chars().nth(index) {
            if let Some(possible_letters) = dict.get(&digit) {
                for letter in possible_letters.chars() {
                    let mut new_path = path.clone();
                    new_path.push(letter);
                    Self::back_track(index + 1, new_path, digits, dict, com);
                }
            }
        }
    }
}
```
