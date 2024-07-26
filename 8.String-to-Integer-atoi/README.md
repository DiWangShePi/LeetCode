# 8. String to Integer (atoi)

### 题目描述
实现 myAtoi(string s) 函数，该函数将字符串转换为 32 位有符号整数。

myAtoi(string s) 的算法如下：

空格：忽略任何前导空格（“ ”）。
符号性：通过检查下一个字符是“-”还是“+”来确定符号，假设不存在正数。
转换：通过跳过前导零来读取整数，直到遇到非数字字符或到达字符串末尾。如果没有读取数字，则结果为 0。
舍入：如果整数超出 32 位有符号整数范围 [-2^31, 2^31 - 1]，则对整数进行舍入以保持在范围内。具体而言，小于 -2^31 的整数应舍入为 -2^31，大于 2^31 - 1 的整数应舍入为 2^31 - 1。
返回整数作为最终结果。

**示例:**

```
Input: s = "42"
Output: 42
Explanation:
The underlined characters are what is read in and the caret is the current reader position.
Step 1: "42" (no characters read because there is no leading whitespace)
         ^
Step 2: "42" (no characters read because there is neither a '-' nor '+')
         ^
Step 3: "42" ("42" is read in)

Input: s = " -042"
Output: -42
Explanation:
Step 1: "   -042" (leading whitespace is read and ignored)
            ^
Step 2: "   -042" ('-' is read, so the result should be negative)
             ^
Step 3: "   -042" ("042" is read in, leading zeros ignored in the result)

Input: s = "1337c0d3"
Output: 1337
Explanation:
Step 1: "1337c0d3" (no characters read because there is no leading whitespace)
         ^
Step 2: "1337c0d3" (no characters read because there is neither a '-' nor '+')
         ^
Step 3: "1337c0d3" ("1337" is read in; reading stops because the next character is a non-digit)

Input: s = "0-1"
Output: 0
Explanation:
Step 1: "0-1" (no characters read because there is no leading whitespace)
         ^
Step 2: "0-1" (no characters read because there is neither a '-' nor '+')
         ^
Step 3: "0-1" ("0" is read in; reading stops because the next character is a non-digit)

Input: s = "words and 987"
Output: 0
Explanation:
Reading stops at the first non-digit character 'w'.
```

### 题目解析

按照题目要求实现即可。

### 代码实现

###### c++
```c++
#include <iostream>
#include <string>
#include <climits>

using namespace std;

class Solution {
public:
    int myAtoi(string s) {
        int i = 0;
        std::string answer;

        while (i < s.length() && s[i] == ' ') {
            i++;
        }

        bool isNei = false;
        if (i < s.length() && s[i] == '-') {
            isNei = true;
            i++;
        } else if (i < s.length() && s[i] == '+') {
            i++;
        }

        while (i < s.length() && '0' <= s[i] && s[i] <= '9') {
            answer += s[i];
            i++;
        }

        if (answer.empty()) {
            return 0;
        }

        try {
            long long result = stoll(answer);
            if (isNei) {
                result = -result;
            }
            if (result > INT_MAX) return INT_MAX;
            if (result < INT_MIN) return INT_MIN;
            return static_cast<int>(result);
        } catch (out_of_range&) {
            return isNei ? INT_MIN : INT_MAX;
        }
    }
};
```

###### rust
```rust
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut i: usize = 0;
        let mut answer = String::new();
        let s_chars: Vec<char> = s.chars().collect();

        // 跳过前导空格
        while i < s_chars.len() && s_chars[i] == ' ' {
            i += 1;
        }

        let mut is_nei: bool = false;
        // 检查符号
        if i < s_chars.len() {
            if s_chars[i] == '-' {
                is_nei = true;
                i += 1;
            } else if s_chars[i] == '+' {
                i += 1;
            }
        }

        // 读取数字字符
        while i < s_chars.len() && s_chars[i].is_digit(10) {
            answer.push(s_chars[i]);
            i += 1;
        }

        if answer.is_empty() {
            return 0;
        }

        match answer.parse::<i32>() {
            Ok(real_answer) => {
                if is_nei {
                    -real_answer
                } else {
                    real_answer
                }
            }
            Err(_) => {
                if is_nei {
                    i32::MIN
                } else {
                    i32::MAX
                }
            }
        }
    }
}
```