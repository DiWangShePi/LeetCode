# 473. Matchsticks to Square

**Tags:** Dynamic Programming, Bitmask

### Description

You are given an integer array matchsticks where matchsticks[i] is the length of the ith matchstick. You want to use all the matchsticks to make one square. You should not break any stick, but you can link them up, and each matchstick must be used exactly one time.

Return true if you can make this square and false otherwise.

### Example 

###### Example I

![](./matchsticks1-grid.jpg)

> Input: matchsticks = [1,1,2,2,2]
> Output: true
> Explanation: You can form a square with length 2, one side of the square came two sticks with length 1.

###### Example II

> Input: matchsticks = [3,3,3,3,4]
> Output: false
> Explanation: You cannot find a way to form a square with all the matchsticks.

### Solution

暴力的解法是先算出边长，然后枚举每一种可能的填补方式

```c++
class Solution {
public:
    bool makesquare(vector<int>& matchsticks) {
        long long sum = accumulate(matchsticks.begin(), matchsticks.end(), 0LL);
        if (sum % 4 != 0) return false;

        long long length = sum / 4;
        vector<long long> edges(4, 0);
        int index = 0;
        sort(matchsticks.begin(), matchsticks.end(), greater<int>());
        return dfs(edges, length, matchsticks, index);
    }

private:
    bool dfs(vector<long long>& edges, long long length, vector<int>& sticks, int index) {
        if (index >= sticks.size()) {
            return (edges[0] == length) && (edges[1] == length) && (edges[2] == length) && (edges[3] == length);
        }

        for (int i = 0; i < 4; i++) {
            if (edges[i] + sticks[index] <= length) {
                edges[i] += sticks[index];

                if (dfs(edges, length, sticks, index + 1)) return true;
                edges[i] -= sticks[index];
            }
        }
        return false;
    }
};
```

官方提供的解法十分有趣，我们可以仅在一条边上的数字填满了之后再填下一条边。
用数字s表示有哪些数字已经被填入到一条边中（第i位为0或者1代表该数字填入与否），这一表示方式使用位掩码实现。
然后我们就可以用动态规划来处理这个问题了，对于dp[s]，假定k是最后放进来的火柴，检测去掉k之后的状态s1是否合法，且该状态下拼得的边长加上k是否超过限制。都满足则s状态也是可行的。
对于指定状态s，只要存在一个可行的解法就可以了。

```c++
class Solution {
public:
    bool makesquare(vector<int>& matchsticks) {
        int totalLen = accumulate(matchsticks.begin(), matchsticks.end(), 0);
        if (totalLen % 4 != 0) {
            return false;
        }
        int len = totalLen / 4, n = matchsticks.size();
        vector<int> dp(1 << n, -1);
        dp[0] = 0;
        for (int s = 1; s < (1 << n); s++) {
            for (int k = 0; k < n; k++) {
                if ((s & (1 << k)) == 0) {
                    continue;
                }
                int s1 = s & ~(1 << k);
                if (dp[s1] >= 0 && dp[s1] + matchsticks[k] <= len) {
                    dp[s] = (dp[s1] + matchsticks[k]) % len;
                    break; // 找到任意一种拼法即可
                }
            }
        }
        return dp[(1 << n) - 1] == 0;
    }
};
```
