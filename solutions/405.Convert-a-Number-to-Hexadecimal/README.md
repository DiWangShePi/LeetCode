# 405. Convert a Number to Hexadecimal

### Descirpiton

Given a 32-bit integer num, return a string representing its hexadecimal representation. For negative integers, two’s complement method is used.

All the letters in the answer string should be lowercase characters, and there should not be any leading zeros in the answer except for the zero itself.

Note: You are not allowed to use any built-in library method to directly solve this problem.

### Example 

###### Example I

> Input: num = 26
> Output: "1a"

###### Example II

> Input: num = -1
> Output: "ffffffff"

### Solution

四个一组，转换为十六进制

```c++
class Solution {
public:
    string toHex(int num) {
        if (num == 0) return "0";
        
        string sb;
        for (int i = 7; i >= 0; i --) {
            int val = (num >> (4 * i)) & 0xf;
            if (sb.length() > 0 || val > 0) {
                char digit = val < 10 ? (char) ('0' + val) : (char) ('a' + val - 10);
                sb.push_back(digit);
            }
        }
        return sb;
    }
};
```
