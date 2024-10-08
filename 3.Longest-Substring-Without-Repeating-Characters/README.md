# 3. Longest Substring Without Repeating Characters

### 题目描述

给定一个字符串，请你找出其中不含有重复字符的 **最长子串** 的长度。

**示例:**

```
给定字符串: "abcabcbb"
返回: 3 
解释: 因为无重复字符的最长子串是 "abc"，所以其长度为 3。
```

### 题目解析

遍历字符串，维护全局哈希表记录出现的字母与其下表，维护变量`length`作为滑动窗口大小，维护变量`result`作为最终结果。

- 初始化空白哈希表与变量`result`、`length`。
- 遍历字母串，针对每个获取的字母，检测其是否存在于哈希表中
  - 不存在，则在哈希表中记录当前字母与其下标
    - 更新`length`为`length+1`
  - 存在，则在哈希表中更新该字母与下标
    - 更新`length`为字母当前下标减去字母前下标
  - 更新`result`为`max(length,result)`


### 代码实现

###### c++

``` c++
class Solution {
public:
    int lengthOfLongestSubstring(string s) {
        std::map<char, int> dict;
        int maxLength = 0, left = 0;

        for (int i = 0; i < s.length(); i++) {
            char currentChar = s[i];

            if (dict.count(currentChar) > 0) {
                left = dict[currentChar] + 1 > left ? dict[currentChar] + 1 : left;
            }

            dict[currentChar] = i;
            maxLength = (i - left + 1) > maxLength ? (i - left + 1) : maxLength;
        }
        return maxLength;
    }
};
```

###### rust

```rust
use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut dict = HashMap::new();
        let mut left: i32 = 0;
        let mut max_length: i32 = 0;

        for i in 0..s.len() {
            let current_char = s[i..=i].to_string();

            if let Some(&index) = dict.get(&current_char) {
                if index as i32 + 1 > left {
                    left = index as i32 + 1;
                }
            }

            dict.insert(current_char, i);
            if i as i32 - left + 1 > max_length {
                max_length = i as i32 - left + 1;
            }
        }

        max_length
    }
}
```