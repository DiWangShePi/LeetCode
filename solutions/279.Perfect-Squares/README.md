# 279. Perfect Squares

### Description

Given an integer n, return the least number of perfect square numbers that sum to n.

A perfect square is an integer that is the square of an integer; in other words, it is the product of some integer with itself. For example, 1, 4, 9, and 16 are perfect squares while 3 and 11 are not.

### Example

###### Example I:

```
Input: n = 12
Output: 3
Explanation: 12 = 4 + 4 + 4.
```

```
Input: n = 13
Output: 2
Explanation: 13 = 4 + 9.
```

### Solution

动态规划，定义当前dp[i]为需要的最小完全平方数，遍历1到根号i（向下取整），更新dp[i]为min(dp[i], dp[i - j * j] + 1)。

```c++
class Solution {
public:
    int numSquares(int n) {
        vector<int> helper;
        for (int i = 0; i <= n; i++) helper.push_back(i);

        for (int i = 1; i <= n; i++) {
            int j = 1;
            while (j * j <= i) {
                helper[i] = min(helper[i], helper[i - j * j] + 1);
                j++;
            }
        }
        return helper[n];
    }
};
```
