# 322. Coin Change

### Description

You are given an integer array coins representing coins of different denominations and an integer amount representing a total amount of money.

Return the fewest number of coins that you need to make up that amount. If that amount of money cannot be made up by any combination of the coins, return -1.

You may assume that you have an infinite number of each kind of coin.

### Example 

###### Example I

```
Input: coins = [1,2,5], amount = 11
Output: 3
Explanation: 11 = 5 + 5 + 1
```

###### Example II

```
Input: coins = [2], amount = 3
Output: -1
```

###### Example III

```
Input: coins = [1], amount = 0
Output: 0
```

### Solution

动态规划，定义dp[i]代表达到i数量的金钱需要的最少的coin数量。遍历0到amount，枚举每一个coin，检查dp[i]与dp[i - coin] + 1谁更小。

```c++
class Solution {
public:
    int coinChange(vector<int>& coins, int amount) {
        vector<int> dp(amount + 1, amount + 1);
        dp[0] = 0;
        for (int i = 0; i < amount + 1; i++) {
            for (int coin : coins) {
                if (coin <= i) dp[i] = min(dp[i], dp[i - coin] + 1);
            }
        }
        return dp[amount] > amount ? -1 : dp[amount];
    }
};
```

> 一开始想用贪心+DFS，但没写出来。这个deepseek的解会超时

```c++
class Solution {
public:
    int coinChange(vector<int>& coins, int amount) {
        sort(coins.rbegin(), coins.rend()); 
        int ans = INT_MAX;
        dfs(coins, amount, 0, 0, ans);
        return ans == INT_MAX ? -1 : ans;
    }

    void dfs(vector<int>& coins, int amount, int index, int count, int& ans) {
        if (amount == 0) {
            ans = min(ans, count);
            return;
        }
        if (index == coins.size()) return;

        for (int k = amount / coins[index]; k >= 0 && count + k < ans; k--) {
            dfs(coins, amount - k * coins[index], index + 1, count + k, ans);
        }
    }
};
```