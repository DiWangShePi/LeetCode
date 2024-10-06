# 65. Valid Number

### 题目描述

给定一个字符串 s，返回 s 是否为有效数字。

例如，以下所有数字都是有效数字：“2”、“0089”、“-0.1”、“+3.14”、“4.”、“-.9”、“2e10”、“-90E3”、“3e+7”、“+6e-1”、“53.5e93”、“-123.456e789”，而以下数字不是有效数字：“abc”、“1a”、“1e”、“e3”、“99e2.5”、“--6”、“-+3”、“95a54e53”。

正式来说，有效数字使用以下定义之一来定义：

整数后跟可选指数。
十进制数后跟可选指数。
整数用可选符号“-”或“+”后跟数字来定义。

十进制数的定义是：可选符号“-”或“+”，后跟以下定义之一：

数字后跟一个点“.”。
数字后跟一个点“.”，后跟数字。
点“.”，后跟数字。
指数的定义是：指数符号“e”或“E”，后跟一个整数。

数字定义为一个或多个数字。

**示例：**

```
Input: s = "0"
Output: true
```

```
Input: s = "e"
Output: false
```

```
Input: s = "."
Output: false
```

### 题目解析

按题目描述实现即可

- 检查字符串开头是否为-或者+。
- 检查是否存在.
- 检查是否存在e或者E
- 检查在存在的.或者e/E前后都有合法的数字
- 检查.或者e/E出现次数不多于一次

### 代码实现

###### c++

```c++
class Solution {
private:
    // This function checks if the rest of the string after 'e' is a valid integer
    bool restValidNumCheck(string &s, int ind) {
        // Skip sign if present
        if (ind < s.length()) {
            if (s[ind] == '-' || s[ind] == '+') ind++;
        }

        bool hasDigit = false;
        
        // Check remaining characters after 'e' or 'E'
        while (ind < s.length()) {
            char ch = s[ind];
            // If it's not a digit, it's invalid
            if (!isdigit(ch)) {
                return false;
            }
            hasDigit = true;  // Mark that a digit has been encountered
            ind++;
        }
        
        return hasDigit;  // Valid if there was at least one digit
    }
    
public:
    bool isNumber(string s) {
        bool decimal_used = false;  // Track if a decimal point has been used
        bool isPrvNum = false;  // Track if there is a valid number before 'e'
        int i = 0;

        // Skip initial sign if present
        if (s[i] == '-' || s[i] == '+') i++;

        for ( ; i < s.length(); i++) {
            char ch = s[i];
            
            // No signs allowed in the middle of the number
            if (ch == '-' || ch == '+') return false;
            
            // Handle alphabetic characters
            else if (isalpha(ch)) {
                if (ch == 'e' || ch == 'E') {
                    // If 'e' is found, check the rest of the string for a valid integer
                    return isPrvNum && restValidNumCheck(s, i + 1);
                } else return false;  // Any other alphabetic character is invalid
            }
            
            // Handle decimal point
            else if (ch == '.') {
                if (decimal_used) return false;  // No multiple decimal points allowed
                else decimal_used = true;  // Mark that a decimal point has been used
            }
            
            // Handle digits
            else isPrvNum = true;  // Mark that we've encountered a digit
        }
        
        // The string is valid only if there was at least one digit
        return isPrvNum;
    }
};
```