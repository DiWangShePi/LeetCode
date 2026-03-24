# 647. Palindromic Substrings

**Tags:** String, DO IT AGAIN

### Description

Given a string s, return the number of palindromic substrings in it.

A string is a palindrome when it reads the same backward as forward.

A substring is a contiguous sequence of characters within the string.

### Example

###### Example I

> Input: s = "abc"
> Output: 3
> Explanation: Three palindromic strings: "a", "b", "c".

###### Example II

> Input: s = "aaa"
> Output: 6
> Explanation: Six palindromic strings: "a", "a", "a", "aa", "aa", "aaa".

### Solution

从每一个可能是回文子串中心点的地方出发，往两边延申，这样的时间复杂度是 O(n^2) 的。如果是枚举所有可能的子串再做检查，就是 O(n^3) 的。

```c++
class Solution {
public:
    int countSubstrings(string s) {
        int n = s.size(), ans = 0;
        for (int i = 0; i < 2 * n - 1; i++) {
            int l = i / 2, r = i / 2 + i % 2;
            while (l >= 0 && r < n && s[l] == s[r]) {
                l--;
                r++;
                ans++;
            }
        }
        return ans;
    }
};
```
