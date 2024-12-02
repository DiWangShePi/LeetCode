# 121. Best Time to Buy and Sell Stock

### Description

You are given an array prices where prices[i] is the price of a given stock on the ith day.

You want to maximize your profit by choosing a single day to buy one stock and choosing a different day in the future to sell that stock.

Return the maximum profit you can achieve from this transaction. If you cannot achieve any profit, return 0.

### Solution

遍历，如果当前值小于最小值，则更新最小值。若当前值大于最小值，则计算差并更新最大差距。

### Implementation

###### c++

```c++
class Solution {
public:
    int maxProfit(vector<int>& prices) {
        int minPrice = INT_MAX;
        int maxProfit = INT_MIN;
        for (int i = 0; i < prices.size(); i++) {
            if (prices[i] < minPrice) minPrice = prices[i];
            int profit = prices[i] - minPrice;
            if (maxProfit < profit) maxProfit = profit;
        }
        return maxProfit;
    }
};
```
