# 123. Best Time to Buy and Sell Stock III

### Description

You are given an array prices where prices[i] is the price of a given stock on the ith day.

Find the maximum profit you can achieve. You may complete at most two transactions.

Note: You may not engage in multiple transactions simultaneously (i.e., you must sell the stock before you buy again).

### Solution

由于不能同时持有多份股票，因此可以依据两笔交易划分时间。第一笔交易为第一个时间段的最大利润交易，第二笔交易为第二个时间段的最大利润交易。

因此，可以考虑遍历数组，对于每一个元素，计算左数组和右数组的最大利润。总和的最大利润即为最终答案。

### Implementation

###### c++

```c++
class Solution {
public:
    int maxProfit(vector<int>& prices) {
        if (prices.empty()) return 0;

        int n = prices.size();
        vector<int> left(n, 0), right(n, 0);

        int min_price = prices[0];
        for (int i = 1; i < n; i++) {
            min_price = min(min_price, prices[i]);
            left[i] = max(left[i - 1], prices[i] - min_price);
        }

        int max_price = prices[n - 1];
        for (int i = n - 2; i >= 0; i--) {
            max_price = max(max_price, prices[i]);
            right[i] = max(right[i + 1], max_price - prices[i]);
        }

        int max_profit = 0;
        for (int i = 0; i < n; i++) {
            max_profit = max(max_profit, left[i] + right[i]);
        }

        return max_profit;
    }
};
```