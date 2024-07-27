# 9. Palindrome Number

### 题目描述

判断一个整数是否是回文数。回文数是指正序（从左向右）和倒序（从右向左）读都是一样的整数。

**示例 1:**

```
输入: 121
输出: true
```

**示例 2:**

```
输入: -121
输出: false
解释: 从左向右读, 为 -121 。 从右向左读, 为 121- 。因此它不是一个回文数。
```

**示例 3:**

```
输入: 10
输出: false
解释: 从右向左读, 为 01 。因此它不是一个回文数。
```

### 题目解析

假设要比较的数字为12321。

首先将12321模10，获得的结果为1232和1。

继续将12321模10，获得的结果为123和2。将上一步获得的1乘以10，再加上2。

再将123模10，获得的结果为12和3，12与此前保留下来的12相等，因此结果为正确。

### 代码实现

###### c++

```c++
#include <string>

using namespace std;

class Solution {
public:
    bool isPalindrome(int x) {
        std::string convertedX = std::to_string(x);
        for (int i = 0; i < convertedX.length(); i++) {
            int reverse = convertedX.length() - i - 1;
            if (reverse < i) {
                break;
            }

            if (convertedX[i] != convertedX[reverse]) {
                return false;
            }
        }
        return true;
    }
};
```