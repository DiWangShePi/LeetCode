# 168. Excel Sheet Column Title

### Description

Given an integer columnNumber, return its corresponding column title as it appears in an Excel sheet.

### Solution

逐步模26求余数

### Implementation

###### c++

```c++
class Solution {
public:
    string convertToTitle(int columnNumber) {
        std::string result;
    
        while (columnNumber > 0) {
            columnNumber--; 
            char ch = 'A' + (columnNumber % 26);
            result = ch + result; 
            columnNumber /= 26; 
        }
        return result;
    }
};
```
