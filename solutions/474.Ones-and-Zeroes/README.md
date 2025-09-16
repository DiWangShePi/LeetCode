# 474. Ones and Zeroes

### Description

You are given an array of binary strings strs and two integers m and n.

Return the size of the largest subset of strs such that there are at most m 0's and n 1's in the subset.

A set x is a subset of a set y if all elements of x are also elements of y.

### Example 

###### Example I

> Input: strs = ["10","0001","111001","1","0"], m = 5, n = 3
> Output: 4
> Explanation: The largest subset with at most 5 0's and 3 1's is {"10", "0001", "1", "0"}, so the answer is 4.
> Other valid but smaller subsets include {"0001", "1"} and {"10", "1", "0"}.
> {"111001"} is an invalid subset because it contains 4 1's, greater than the maximum of 3.

###### Example II

> Input: strs = ["10","0","1"], m = 1, n = 1
> Output: 2
> Explanation: The largest subset is {"0", "1"}, so the answer is 2.

### Solution

动态规划：定义dp[i][j][k]代表覆盖的数组长度为i时，0和1的上界分别为j和k时能容纳的最多的字符串。

遍历给定的字符串列表，得到每一个字符串包含的0和1的个数。枚举j和k，
- 当j和k均不大于当前字符串的0和1的个数时（即当前字符无法被容纳），dp[i][j][k] = dp[i - 1][j][k]。
- 当j和k均大于当前字符串的个数时，考虑容纳当前字符串，取dp[i - 1][j][k]和dp[i - 1][j - zeros][k - ones] + 1的较大值。

```c++
class Solution {
public:
    int findMaxForm(vector<string>& strs, int m, int n) {
        int l = strs.size();
        vector<int> containsOne(l, 0);
        vector<int> containsZero(l, 0);

        for (int i = 0; i < l; i++) {
            string s = strs[i];
            for (char c : s) {
                if (c == '0') containsZero[i]++;
                else if (c == '1') containsOne[i]++;
            }
        }

        vector<vector<vector<int>>> dp(l + 1, vector<vector<int>>(m + 1, vector<int>(n + 1, 0)));
        for (int i = 1; i <= l; i++) {
            int zeros = containsZero[i - 1];
            int ones = containsOne[i - 1];

            for (int j = 0; j <= m; j++) {
                for (int k = 0; k <= n; k++) {
                    dp[i][j][k] = dp[i - 1][j][k];
                    if (j >= zeros && k >= ones) {
                        dp[i][j][k] = max(dp[i][j][k], dp[i - 1][j - zeros][k - ones] + 1);
                    }
                }
            }
        }
        return dp[l][m][n];
    }
};
```

由于每次dp只依赖前一次的状态，因此我们无需保存所有的状态，只需要保存上一次的就可以了

```c++
class Solution {
public:
    int findMaxForm(vector<string>& strs, int m, int n) {
        vector<vector<int>> dp(m + 1, vector<int>(n + 1, 0));
        
        for (const string& s : strs) {
            int zeros = 0, ones = 0;
            for (char c : s) {
                if (c == '0') zeros++;
                else ones++;
            }
            
            for (int j = m; j >= zeros; j--) {
                for (int k = n; k >= ones; k--) {
                    dp[j][k] = max(dp[j][k], dp[j - zeros][k - ones] + 1);
                }
            }
        }
        
        return dp[m][n];
    }
};
```
