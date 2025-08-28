# 459. Repeated Substring Pattern

**Tags:** String, KMP

### Description

Given a string s, check if it can be constructed by taking a substring of it and appending multiple copies of the substring together.

### Example

###### Example I

> Input: s = "abab"
> Output: true
> Explanation: It is the substring "ab" twice.

###### Example II

> Input: s = "aba"
> Output: false

###### Example III

> Input: s = "abcabcabcabc"
> Output: true
> Explanation: It is the substring "abc" four times or the substring "abcabc" twice.

### Solution

先试试暴力枚举，尝试所有可能的i（能被n整除， 且不为n本身）。

```c++
class Solution {
public:
    bool repeatedSubstringPattern(string s) {
        int n = s.size();
        for (int i = 1; i * 2 <= n; i++) {
            if (n % i == 0) {
                bool match = true;
                for (int j = i; j < n; j++) {
                    if (s[j] != s[j - i]) {
                        match = false;
                        break;
                    }
                }
                if (match) return true;
            }
        }
        return false;
    }
};
```
