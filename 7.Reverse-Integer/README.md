# 7. Reverse Integer

### 题目描述

给定一个有符号的 32 位整数 x，返回数字反转后的 x。如果反转 x 导致值超出有符号的 32 位整数范围 [-231, 231 - 1]，则返回 0。

**示例:**

```
Input: x = 123
Output: 321

Input: x = -123
Output: -321

Input: x = 120
Output: 21
```

### 题目解析

如果输入数字x为INT_MIN，则直接返回0。

使用变量isNei记录x是否为负值，随后将x转换为正值，再转换为字符串。

倒过来遍历字符串，用变量isZero记录是否略过了开头的0。将新的字符串拼接起来。

遍历结束后，转换为Int类型数字。根据isNei确定是否变为负值。


### 代码实现

```c++
#include <climits>
#include <string>

using namespace std;

class Solution {
public:
    int reverse(int x) {
        if (x == INT_MIN) {
            return 0;
        }

        bool isNei = false;
        if (x < 0) {
            isNei = true;
            x = -x;
        }

        std::string convertedX = std::to_string(x);
        std::string possibleAn = "";
        bool isZero = true;
        for (int i = 0; i < convertedX.length(); i++) {
            int currentPos = convertedX.length() - i - 1;
            if (convertedX[currentPos] == 0 && isZero) {
                continue;
            }

            isZero = false;
            possibleAn += convertedX[currentPos];
        }

        long answer = std::stol(possibleAn);
        if (answer > INT_MAX) {
            return 0;
        }
        return isNei ? - std::stoi(possibleAn) : std::stoi(possibleAn);
    }
};
```

###### rust

```rust
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        if x == i32::MIN { return 0; }

        let mut real_x = x;

        let mut is_nei: bool = false;
        if real_x < 0 {
            is_nei = true;
            real_x = -real_x;
        }

        let converted_x: Vec<char> = real_x.to_string().chars().collect();
        let mut possible_an = String::new();
        let mut is_zero = true;
        for &ch in converted_x.iter().rev() {
            if ch == '0' && is_zero {
                continue;
            }
            
            is_zero = false;
            possible_an.push(ch);
        }

        let answer = possible_an.parse::<i64>().unwrap_or(0);
        if answer > i32::MAX as i64 {
            return 0;
        }
        
        if is_nei {
            return - answer as i32;
        }
        return answer as i32;
    }
}
```