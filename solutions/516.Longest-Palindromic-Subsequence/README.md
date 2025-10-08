# 516. Longest Palindromic Subsequence

**Tags:** Dynamic Programming

### Description

Given a string s, find the longest palindromic subsequence's length in s.

A subsequence is a sequence that can be derived from another sequence by deleting some or no elements without changing the order of the remaining elements.

### Example 

###### Example I

> Input: s = "bbbab"
> Output: 4
> Explanation: One possible longest palindromic subsequence is "bbbb".

###### Example II

> Input: s = "cbbd"
> Output: 2
> Explanation: One possible longest palindromic subsequence is "bb".

### Solution

动态规划。定义dp[i][j]代表s[i:j]中能获得的最长回文子字符串。

- 如果s[i] == s[j]，那么dp[i][j] = dp[i + 1][j - 1] + 2;
- 如果s[i] != s[j]，那么dp[i][j] = max(dp[i + 1][j], dp[i][j - 1]);

考虑到这个状态转移函数，我们需要倒过来遍历i，正着遍历j

```c++
class Solution {
public:
    int longestPalindromeSubseq(string s) {
        int n = s.size();
        vector<vector<int>> dp(n, vector<int>(n, 0));
        for (int i = n - 1; i > -1; i--) {
            dp[i][i] = 1;

            for (int j = i + 1; j < n; j++) {
                if (s[i] == s[j]) dp[i][j] = dp[i + 1][j - 1] + 2;
                else dp[i][j] = max(dp[i][j - 1], dp[i + 1][j]);
            }
        }
        return dp[0][n - 1];
    }
};
```
