# 258. Add Digits

### Description

Given an integer num, repeatedly add all its digits until the result has only one digit, and return it.

### Solution

按要求实现即可

```c++
class Solution {
public:
    int addDigits(int num) {
        int result = num;
        while (result > 9) {
            string current = to_string(result);
            result = 0;
            for (int i = 0; i < current.size(); i++) result += current[i] - '0';
        }
        return result;
    }
};
```
