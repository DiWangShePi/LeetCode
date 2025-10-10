# 518. Coin Change II

**Tags:** Dynamic Programming

### Description

You are given an integer array coins representing coins of different denominations and an integer amount representing a total amount of money.

Return the number of combinations that make up that amount. If that amount of money cannot be made up by any combination of the coins, return 0.

You may assume that you have an infinite number of each kind of coin.

The answer is guaranteed to fit into a signed 32-bit integer.

### Example 

###### Example I

> Input: amount = 5, coins = [1,2,5]
> Output: 4
> Explanation: there are four ways to make up the amount:
> 5=5
> 5=2+2+1
> 5=2+1+1+1
> 5=1+1+1+1+1

###### Example II

> Input: amount = 3, coins = [2]
> Output: 0
> Explanation: the amount of 3 cannot be made up just with coins of 2.

###### Example III

> Input: amount = 10, coins = [10]
> Output: 1

### Solution

动态规划

> 这其实就是爬楼梯

```c++
class Solution {
public:
    int change(int amount, vector<int>& coins) {
        vector<unsigned long long> dp(amount + 1, 0LL);
        dp[0] = 1;

        for (int i = 0; i < coins.size(); i++) {
            int coin = coins[i];

            for (int j = coin; j <= amount; j++) {
                dp[j] += dp[j - coin];
            }
        }
        return dp[amount];
    }
};
```

### Related "Work"

1. LeetCode 322. Coin Change

2. LeetCode 70. Climbing Stairs
