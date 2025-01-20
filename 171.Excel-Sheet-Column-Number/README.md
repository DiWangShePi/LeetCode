# 171. Excel Sheet Column Number

### Description

Given a string columnTitle that represents the column title as appears in an Excel sheet, return its corresponding column number.

### Solution

按要求完成即可

### Implementation

###### c++

```c++
class Solution {
public:
    int titleToNumber(string columnTitle) {
        int result = 0;
        for (char c : columnTitle) {
            result = result * 26 + (c - 'A' + 1);
        }
        return result;
    }
};
```
