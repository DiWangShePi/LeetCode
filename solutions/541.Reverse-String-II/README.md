# 541. Reverse String II

**Tags:** String

### Description

Given a string s and an integer k, reverse the first k characters for every 2k characters counting from the start of the string.

If there are fewer than k characters left, reverse all of them. If there are less than 2k but greater than or equal to k characters, then reverse the first k characters and leave the other as original.

### Example 

###### Example I

> Input: s = "abcdefg", k = 2
> Output: "bacdfeg"

###### Example II

> Input: s = "abcd", k = 2
> Output: "bacd"

### Solution

按要求实现即可

```c++
class Solution {
public:
    string reverseStr(string s, int k) {
        int n = s.size();
        for (int i = 0; i < n; i += 2 * k) {
            int end = min(i + k, n);
            reverse(s.begin() + i, s.begin() + end);
        }
        return s;
    }
};
```
