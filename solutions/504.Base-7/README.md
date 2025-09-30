# 504. Base 7

**Tags:** Math

### Description

Given an integer num, return a string of its base 7 representation.

### Example 

###### Example I

> Input: num = 100
> Output: "202"

###### Example II

> Input: num = -7
> Output: "-10"

### Solution

按要求实现即可

```c++
class Solution {
public:
    string convertToBase7(int num) {
        if (num == 0) return "0";

        bool is_n = num >= 0 ? false: true;
        if (is_n) num = -num;
        string an = "";
        while (num != 0) {
            char t = (num % 7) + '0';
            an = t + an;

            num /= 7;
        }
        return is_n ? "-" + an : an;
    }
};
```
