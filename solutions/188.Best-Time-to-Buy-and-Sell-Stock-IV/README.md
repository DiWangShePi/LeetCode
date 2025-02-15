# 188. Best Time to Buy and Sell Stock IV

### Desctipition

You are given an integer array prices where prices[i] is the price of a given stock on the ith day, and an integer k.

Find the maximum profit you can achieve. You may complete at most k transactions: i.e. you may buy at most k times and sell at most k times.

Note: You may not engage in multiple transactions simultaneously (i.e., you must sell the stock before you buy again).

### Solution

在此前第123题中，我们限定了k=2，所以我们可以将数组划分为左右两个部分。分别求取两部分的最大利润，之和即为整个数组的最大利润。这题就不能这样了。

参考 https://leetcode.cn/problems/best-time-to-buy-and-sell-stock-iv/solutions/537731/mai-mai-gu-piao-de-zui-jia-shi-ji-iv-by-8xtkp/ 下子不语的题解

### Implementation

###### c++

```c++
class Solution {
public:
    int maxProfit(int k, vector<int>& prices) {
        if (prices.empty()) {
            return 0;
        }

        int n = prices.size();
        k = min(k, n / 2);
        vector<int> buy(k + 1);
        vector<int> sell(k + 1);

        buy[0] = -prices[0];
        sell[0] = 0;
        for (int i = 1; i <= k; ++i) {
            buy[i] = sell[i] = INT_MIN / 2;
        }

        for (int i = 1; i < n; ++i) {
            buy[0] = max(buy[0], sell[0] - prices[i]);
            for (int j = 1; j <= k; ++j) {
                sell[j] = max(sell[j], buy[j - 1] + prices[i]);   
                buy[j] = max(buy[j], sell[j] - prices[i]);
            }
        }

        return sell.back();
    }
};
```

> 哎，感觉这样刷题成效不行啊
