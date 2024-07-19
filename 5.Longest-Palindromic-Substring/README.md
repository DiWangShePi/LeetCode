# 5. Longest Palindromic Substring

### 题目描述

给定一个字符串，要求这个字符串当中最长的回文串。

**示例1:**

```
Input: "babad"
Output: "bab"
Note: "aba" is also a valid answer.
```

**示例2:**
```
Input: "cbbd"
Output: "bb"
```

### 题目解析

参考马拉车算法：https://en.wikipedia.org/wiki/Longest_palindromic_substring#Manacher's_algorithm


### 代码实现

###### c++

```c++
class Solution {
public:
    std::string longestPalindrome(std::string s) {
        if (s.length() <= 1) {
            return s;
        }
        
        int maxLen = 1;
        std::string maxStr = s.substr(0, 1);
        s = "#" + std::regex_replace(s, std::regex(""), "#") + "#";
        std::vector<int> array(s.length(), 0);
        int center = 0;
        int right = 0;
        
        for (int i = 0; i < s.length(); ++i) {
            if (i < right) {
                array[i] = std::min(right - i, array[2 * center - i]);
            }
            
            while (i - array[i] - 1 >= 0 && i + array[i] + 1 < s.length() && s[i - array[i] - 1] == s[i + array[i] + 1]) {
                array[i]++;
            }
            
            if (i + array[i] > right) {
                center = i;
                right = i + array[i];
            }
            
            if (array[i] > maxLen) {
                maxLen = array[i];
                maxStr = s.substr(i - array[i], 2 * array[i] + 1);
                maxStr.erase(std::remove(maxStr.begin(), maxStr.end(), '#'), maxStr.end());
            }
        }
        
        return maxStr;
    }
};
```

###### rust

```rust
use std::cmp::min;
use regex::Regex;

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.len() <= 1 {
            return s.to_string();
        }
    
        let mut max_len = 1;
        let mut max_str = s.chars().next().unwrap().to_string();
        let re = Regex::new("").unwrap();
        let replaced_s = re.replace_all(&s, "#");
        let mut s: String = format!("#{}#", replaced_s);
        let mut array = vec![0; s.len()];
        let mut center = 0;
        let mut right = 0;
    
        for i in 0..s.len() {
            if i < right {
                array[i] = min(right - i, array[2 * center - i]);
            }
    
            while i >= array[i] + 1 && i + array[i] + 1 < s.len() && s.chars().nth(i - array[i] - 1) == s.chars().nth(i + array[i] + 1) {
                array[i] += 1;
            }
    
            if i + array[i] > right {
                center = i;
                right = i + array[i];
            }
    
            if array[i] > max_len {
                max_len = array[i];
                let start = i - array[i];
                let end = i + array[i] + 1;
                max_str = s[start..end].replace("#", "");
            }
        }
    
        max_str
    }
}
```