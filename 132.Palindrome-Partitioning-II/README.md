# 132. Palindrome Partitioning II

### Description

Given a string s, partition s such that every substring of the partition is a palindrome.

Return the minimum cuts needed for a palindrome partitioning of s.

### Solution

定义f[i,j]为字符串s[i:j]的最小分割数，则该值有两种可能：
- 0： 即s[i:j]就是回文字符串，不需要再做分割。
- f[i,k]+1: 即s[i:k]最小分割数加上一次分割，s[k+1:j]为回文字符串。
> 考虑到第二种情况下，获取最长的回文串并不等同于获取最短的分割方式，我们实际上需要遍历并取最小值。

于是，这个问题变成了如何迅速确定指定字符串是回文字符串。这可以采用与上一题类似的方式进行预处理。

### Implementation

###### c++

```c++
class Solution {
public:
    int minCut(string s) {
        int n = s.size();
        vector<vector<int>> g(n, vector<int>(n, true));

        for (int i = n - 1; i >= 0; --i) {
            for (int j = i + 1; j < n; ++j) {
                g[i][j] = (s[i] == s[j]) && g[i + 1][j - 1];
            }
        }

        vector<int> f(n, INT_MAX);
        for (int i = 0; i < n; ++i) {
            if (g[0][i]) {
                f[i] = 0;
            }
            else {
                for (int j = 0; j < i; ++j) {
                    if (g[j + 1][i]) {
                        f[i] = min(f[i], f[j] + 1);
                    }
                }
            }
        }

        return f[n - 1];
    }
};
```