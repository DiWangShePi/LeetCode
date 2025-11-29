# 1208. Get Equal Substrings Within Budget

**Tags:** Sliding Window

### Description

You are given two strings s and t of the same length and an integer maxCost.

You want to change s to t. Changing the ith character of s to ith character of t costs |s[i] - t[i]| (i.e., the absolute difference between the ASCII values of the characters).

Return the maximum length of a substring of s that can be changed to be the same as the corresponding substring of t with a cost less than or equal to maxCost. If there is no substring from s that can be changed to its corresponding substring from t, return 0.

### Example

###### Example I

> Input: s = "abcd", t = "bcdf", maxCost = 3
> Output: 3
> Explanation: "abc" of s can change to "bcd".
> That costs 3, so the maximum length is 3.

###### Example II

> Input: s = "abcd", t = "cdef", maxCost = 3
> Output: 1
> Explanation: Each character in s costs 2 to change to character in t,  so the maximum length is 1.

###### Example III

> Input: s = "abcd", t = "acde", maxCost = 0
> Output: 1
> Explanation: You cannot make any change, so the maximum length is 1.

### Solution

滑动窗口，计算窗口内字符的总开销，与目标开销进行比较，记录最长的子字符串。

```c++
class Solution {
public:
    int equalSubstring(string s, string t, int maxCost) {
        int l = 0, c = 0, an = 0, n = s.size();
        for (int i = 0; i < n; i++) {
            c += abs(s[i] - t[i]);
            while (c > maxCost) {
                c -= abs(s[l] - t[l]);
                l++;
            }

            an = max(an, i - l + 1);
        }
        return max(an, n - l);
    }
};
```
