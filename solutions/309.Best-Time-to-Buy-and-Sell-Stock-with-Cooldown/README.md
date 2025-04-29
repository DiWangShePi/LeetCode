# 309. Best Time to Buy and Sell Stock with Cooldown

### Description

You are given an array prices where prices[i] is the price of a given stock on the ith day.

Find the maximum profit you can achieve. You may complete as many transactions as you like (i.e., buy one and sell one share of the stock multiple times) with the following restrictions:

- After you sell your stock, you cannot buy stock on the next day (i.e., cooldown one day).
Note: You may not engage in multiple transactions simultaneously (i.e., you must sell the stock before you buy again).

### Example 

###### Example I

```
Input: prices = [1,2,3,0,2]
Output: 3
Explanation: transactions = [buy, sell, cooldown, buy, sell]
```

```
Input: prices = [1]
Output: 0
```

### Solution

定义三种状态，`HOLD`代表当前持有股票，`COOL`代表卖出持有的股票，进入冷却，`SELL`代表当前不持有股票。

- `HOLD`状态可以来自于两种，要么昨天也持有，要么昨天不持有，今天买入。
- `COOL`状态只可能来自于昨天持有股票，且今天卖出
- `SELL`状态可能是昨天卖出了，或者昨天也不持有

由此即可构建动态规划

```c++
class Solution {
public:
    int maxProfit(vector<int>& prices) {
        if (prices.empty()) return 0;

        int n = prices.size();
        vector<vector<int>>dp(n, vector<int>(3, 0));
        dp[0][0] = -prices[0];
        for (int i = 1; i < n; i++) {
            dp[i][0] = max(dp[i - 1][0], dp[i - 1][2] - prices[i]);
            dp[i][1] = dp[i - 1][0] + prices[i];
            dp[i][2] = max(dp[i - 1][1], dp[i - 1][2]);
        }
        return max(dp[n-1][1], dp[n-1][2]);
    }
};
```

节省空间的写法
> 显然，我们用于表述状态的属性具有马尔可夫性质

```c++
class Solution {
public:
    int maxProfit(vector<int>& prices) {
        if (prices.empty()) return 0;

        int n = prices.size();
        vector<vector<int>>dp(n, vector<int>(3, 0));
        int last_dp0 = -prices[0], last_dp1 = 0, last_dp2 = 0;
        int dp0 = 0, dp1 = 0, dp2 = 0;
        for (int i = 1; i < n; i++) {
            dp0 = max(last_dp0, last_dp2 - prices[i]);
            dp1 = last_dp0 + prices[i];
            dp2 = max(last_dp1, last_dp2);

            last_dp0 = dp0;
            last_dp1 = dp1;
            last_dp2 = dp2;
        }
        return max(last_dp1, last_dp2);
    }
};
```
