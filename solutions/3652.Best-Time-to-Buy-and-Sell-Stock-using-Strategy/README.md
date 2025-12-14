# 3652. Best Time to Buy and Sell Stock using Strategy

**Tags:** Sliding Window

### Description

You are given two integer arrays prices and strategy, where:

- prices[i] is the price of a given stock on the ith day.
- strategy[i] represents a trading action on the ith day, where:
- - -1 indicates buying one unit of the stock.
- - 0 indicates holding the stock.
- - 1 indicates selling one unit of the stock.

You are also given an even integer k, and may perform at most one modification to strategy. A modification consists of:

- Selecting exactly k consecutive elements in strategy.
- Set the first k / 2 elements to 0 (hold).
- Set the last k / 2 elements to 1 (sell).
The profit is defined as the sum of strategy[i] * prices[i] across all days.

Return the maximum possible profit you can achieve.

Note: There are no constraints on budget or stock ownership, so all buy and sell operations are feasible regardless of past actions.

### Example

###### Example I

> Input: prices = [4,2,8], strategy = [-1,0,1], k = 2
> Output: 10

###### Example II

> Input: prices = [5,4,3], strategy = [1,1,0], k = 2
> Output: 9

### Solution

相当于两个滑动窗口。需要考虑不进行策略变动的收益

```c++
class Solution {
public:
    long long maxProfit(vector<int>& prices, vector<int>& strategy, int k) {
        // try modify
        long long initial = 0;
        int n = prices.size();
        for (int i = k; i < n; i++)
            initial += prices[i] * strategy[i];
        for (int i = 0; i < k; i++) {
            if (i < k / 2) initial += 0 * prices[i];
            else initial += 1 * prices[i];
        }
        
        long long an = initial;
        for (int i = k; i < n; i++) {
            initial -= 0 * prices[i - k];
            initial += prices[i - k] * strategy[i - k];
            
            initial -= prices[i] * strategy[i];
            initial += 1 * prices[i];

            initial -= 1 * prices[i - k / 2];
            an = max(an, initial);
        }

        // do not modify
        long long sum = 0;
        for (int i = 0; i < n; i++)
            sum += prices[i] * strategy[i];

        // return best reward
        return max(an, sum);
    }
};
```
