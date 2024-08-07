# 13. Roman to Integer

### 题目描述

罗马数字包含以下七种字符: I， V， X， L，C，D 和 M。

字符          数值
I             1
V             5
X             10
L             50
C             100
D             500
M             1000
例如， 罗马数字 2 写做 II ，即为两个并列的 1 。12 写做 XII ，即为 X + II 。 27 写做  XXVII, 即为 XX + V + II 。

通常情况下，罗马数字中小的数字在大的数字的右边。但也存在特例，例如 4 不写做 IIII，而是 IV。数字 1 在数字 5 的左边，所表示的数等于大数 5 减小数 1 得到的数值 4 。同样地，数字 9 表示为 IX。这个特殊的规则只适用于以下六种情况：

I 可以放在 V (5) 和 X (10) 的左边，来表示 4 和 9。
X 可以放在 L (50) 和 C (100) 的左边，来表示 40 和 90。 
C 可以放在 D (500) 和 M (1000) 的左边，来表示 400 和 900。
给定一个罗马数字，将其转换成整数。

**示例：**

```
输入: s = "III"
输出: 3
```

```
输入: s = "LVIII"
输出: 58
解释: L = 50, V= 5, III = 3.
```

```
输入: s = "MCMXCIV"
输出: 1994
解释: M = 1000, CM = 900, XC = 90, IV = 4.
```

### 题目解析

通常情况下，罗马数字中小的数字在大的数字的右边。若输入的字符串满足该情况，那么可以将每个字符视作一个单独的值，累加每个字符对应的数值即可。

例如 XXVII 可视作 X+X+V+I+I=10+10+5+1+1=27。

若存在小的数字在大的数字的左边的情况，根据规则需要减去小的数字。对于这种情况，我们也可以将每个字符视作一个单独的值，若一个数字右侧的数字比它大，则将该数字的符号取反。

例如 XIV 可视作 X−I+V=10−1+5=14。

### 代码实现

###### c++

```c++
#include <string>
#include <map>


class Solution {
public:
    int romanToInt(string s) {
        std::map<char, int> dict = {
            {'I', 1},
            {'V', 5},
            {'X', 10},
            {'L', 50},
            {'C', 100},
            {'D', 500},
            {'M', 1000}
        };
        
        int result = 0;
        for (int i = 0; i < s.length(); i++) {
            char currentNum = s[i];

            bool shouldRe = false;
            switch (currentNum)
            {
            case 'I':
                if (i+1 < s.length() && (s[i+1] == 'V' || s[i+1] == 'X')) {
                    shouldRe = true;
                }
                break;
            case 'X':
                if (i+1 < s.length() && (s[i+1] == 'L' || s[i+1] == 'C')) {
                    shouldRe = true;
                }
                break;
            case 'C':
                if (i+1 < s.length() && (s[i+1] == 'D' || s[i+1] == 'M')) {
                    shouldRe = true;
                }
                break;
            default:
                break;
            }

            result = shouldRe ? result - dict[currentNum] : result + dict[currentNum];
        }
        return result;
    }
};
```

###### rust

```rust
use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let dict: HashMap<char, i32> = vec![
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ].into_iter().collect();

        let mut result = 0;
        let chars: Vec<char> = s.chars().collect();
        
        for (i, &current_char) in chars.iter().enumerate() {
            let mut should_re = false;

            match current_char {
                'I' => {
                    if i + 1 < s.len() && (chars[i + 1] == 'V' || chars[i + 1] == 'X') {
                        should_re = true;
                    }
                },
                'X' => {
                    if i + 1 < s.len() && (chars[i + 1] == 'L' || chars[i + 1] == 'C') {
                        should_re = true;
                    }
                },
                'C' => {
                    if i + 1 < s.len() && (chars[i + 1] == 'D' || chars[i + 1] == 'M') {
                        should_re = true;
                    }
                },
                _ => {}
            }

            if should_re {
                result -= dict[&current_char];
            } else {
                result += dict[&current_char];
            }
        }
        
        result
    }
}
```