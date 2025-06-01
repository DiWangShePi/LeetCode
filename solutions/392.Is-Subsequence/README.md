# 392. Is Subsequence

### Description

Given two strings s and t, return true if s is a subsequence of t, or false otherwise.

A subsequence of a string is a new string that is formed from the original string by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters. (i.e., "ace" is a subsequence of "abcde" while "aec" is not).

### Example 

###### Example I

```
Input: s = "abc", t = "ahbgdc"
Output: true
```

###### Example II

```
Input: s = "axc", t = "ahbgdc"
Output: false
```

### Solution

遍历

```c++
class Solution {
public:
    bool isSubsequence(string s, string t) {
        int i = 0, j = 0;
        while (i < s.size() && j < t.size()) {
            while ( j < t.size() && s[i] != t[j]) j++;
            if (s[i] == t[j]) i++; j++;
        }
        return i == s.size();
    }
};
```
