# 125. Valid Palindrome

### Description

A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward. Alphanumeric characters include letters and numbers.

Given a string s, return true if it is a palindrome, or false otherwise.

### Solution

遍历字符串以去除所有非字母的字符，再将大写字符统一转换成小写字符。得到的字符串检查是否符合要求即可。

### Implementation

###### c++

```c++
class Solution {
public:
    bool isPalindrome(string s) {
        std::string filtered = "";
        for (char c : s) {
            if (std::isalnum(c)) {
                filtered += std::tolower(c);
            }
        }

        int left = 0, right = filtered.size() - 1;
        while (left < right) {
            if (filtered[left] != filtered[right]) {
                return false; 
            }
            left++;
            right--;
        }

        return true;
    }
};
```