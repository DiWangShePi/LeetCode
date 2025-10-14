# 521. Longest Uncommon Subsequence I

**Tags:** String

### Description

Given two strings a and b, return the length of the longest uncommon subsequence between a and b. If no such uncommon subsequence exists, return -1.

An uncommon subsequence between two strings is a string that is a subsequence of exactly one of them.

### Example 

###### Example I

> Input: a = "aba", b = "cdc"
> Output: 3
> Explanation: One longest uncommon subsequence is "aba" because "aba" is a subsequence of "aba" but not "cdc".
> Note that "cdc" is also a longest uncommon subsequence.

###### Example II

> Input: a = "aaa", b = "bbb"
> Output: 3
> Explanation: The longest uncommon subsequences are "aaa" and "bbb".

###### Example III

> Input: a = "aaa", b = "aaa"
> Output: -1
> Explanation: Every subsequence of string a is also a subsequence of string b. Similarly, every subsequence of string b is also a subsequence of string a. So the answer would be -1.

### Solution

如果 a 和 b 相等，那么返回 -1。如果 a 和 b 不相等，那么返回较长的哪一个。

```c++
class Solution {
public:
    int findLUSlength(string a, string b) {
        return a != b ? max(a.size(), b.size()) : -1;
    }
};
```

### Related "Works"

1. LeetCode 522. Longest Uncommon Subsequence II
