# 122. Best Time to Buy and Sell Stock II

### Description

You are given an integer array prices where prices[i] is the price of a given stock on the ith day.

On each day, you may decide to buy and/or sell the stock. You can only hold at most one share of the stock at any time. However, you can buy it then immediately sell it on the same day.

Find and return the maximum profit you can achieve.

### Solution

对数组从1开始遍历，若当前值减去上一个值大于0，则加入到回报中，小于0则略过。

### Implementation

###### c++

```c++
class Solution {
public:
    int maxProfit(vector<int>& prices) {
        int result = 0;
        for (int i = 1; i < prices.size(); i++) {
            int possible = prices[i] - prices[i - 1];
            if (possible > 0) result += possible;
        }
        return result;
    }
};
```