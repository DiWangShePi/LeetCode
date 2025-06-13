# 375. Guess Number Higher or Lower II

### Description

We are playing the Guessing Game. The game will work as follows:

1. I pick a number between 1 and n.
2. You guess a number.
3. If you guess the right number, you win the game.
4. If you guess the wrong number, then I will tell you whether the number I picked is higher or lower, and you will continue guessing.

Every time you guess a wrong number x, you will pay x dollars. If you run out of money, you lose the game.
Given a particular n, return the minimum amount of money you need to guarantee a win regardless of what number I pick.

### Example 

###### Example I

![](./graph.png)

```
Input: n = 10
Output: 16
Explanation: The winning strategy is as follows:
- The range is [1,10]. Guess 7.
    - If this is my number, your total is $0. Otherwise, you pay $7.
    - If my number is higher, the range is [8,10]. Guess 9.
        - If this is my number, your total is $7. Otherwise, you pay $9.
        - If my number is higher, it must be 10. Guess 10. Your total is $7 + $9 = $16.
        - If my number is lower, it must be 8. Guess 8. Your total is $7 + $9 = $16.
    - If my number is lower, the range is [1,6]. Guess 3.
        - If this is my number, your total is $7. Otherwise, you pay $3.
        - If my number is higher, the range is [4,6]. Guess 5.
            - If this is my number, your total is $7 + $3 = $10. Otherwise, you pay $5.
            - If my number is higher, it must be 6. Guess 6. Your total is $7 + $3 + $5 = $15.
            - If my number is lower, it must be 4. Guess 4. Your total is $7 + $3 + $5 = $15.
        - If my number is lower, the range is [1,2]. Guess 1.
            - If this is my number, your total is $7 + $3 = $10. Otherwise, you pay $1.
            - If my number is higher, it must be 2. Guess 2. Your total is $7 + $3 + $1 = $11.
The worst case in all these scenarios is that you pay $16. Hence, you only need $16 to guarantee a win.
```

###### Example II

```
Input: n = 1
Output: 0
Explanation: There is only one possible number, so you can guess 1 and not have to pay anything.
```

###### Example III

```
Input: n = 2
Output: 1
Explanation: There are two possible numbers, 1 and 2.
- Guess 1.
    - If this is my number, your total is $0. Otherwise, you pay $1.
    - If my number is higher, it must be 2. Guess 2. Your total is $1.
The worst case is that you pay $1.
```

### Solution

这个问题可以使用动态规划（Dynamic Programming, DP）来解决。我们需要定义一个状态，然后找到状态之间的转移关系。

###### 定义状态：
设dp[i][j]表示在数字范围[i, j]内猜中任意数字所需的最小成本（即最少需要准备的钱）。

###### 目标：
我们需要计算dp[1][n]，即在数字1到n范围内猜中任意数字的最小成本。

###### 基本情况：

- 如果i == j，即范围内只有一个数字，那么你直接猜这个数字，猜中，不需要支付任何成本（因为猜中不支付），所以dp[i][i] = 0。

- 如果i > j，这种情况不存在，可以视为0。

###### 状态转移：
对于范围[i, j]，我们可以选择其中的任意一个数字k（i <= k <= j）作为猜测：

1. 如果k是正确的数字，那么成本为0（因为猜中了）。

2. 如果k不是正确的数字：

3. 如果正确数字比k小，那么我们需要在[i, k-1]范围内继续猜，成本为k + dp[i][k-1]。

4. 如果正确数字比k大，那么我们需要在[k+1, j]范围内继续猜，成本为k + dp[k+1][j]。

为了确保在最坏情况下能够覆盖所有可能性，我们需要选择k使得最坏情况下的成本最小。因此，对于每个k，最坏成本是k + max(dp[i][k-1], dp[k+1][j])。然后我们需要选择k使得这个最坏成本最小：

dp[i][j] = min_{k} (k + max(dp[i][k-1], dp[k+1][j])) for all k in [i, j]

###### 初始化：

dp[i][i] = 0，因为只有一个数字时直接猜中。

###### 计算顺序：

我们需要按子区间的长度从小到大计算dp[i][j]。即先计算所有长度为1的区间，然后长度为2，...，直到长度为n。

```c++
class Solution {
public:
    int getMoneyAmount(int n) {
        // dp[i][j] represents the minimum cost to guarantee a win in the range [i, j]
        std::vector<std::vector<int>> dp(n + 2, std::vector<int>(n + 2, 0));

        for (int length = 2; length <= n; ++length) {
            for (int i = 1; i <= n - length + 1; ++i) {
                int j = i + length - 1;  // Calculate the end of the current subarray
                dp[i][j] = INT_MAX;      // Initialize to a large value

                for (int k = i; k <= j; ++k) {
                    int cost = k + std::max(dp[i][k - 1], dp[k + 1][j]);
                    if (cost < dp[i][j]) {
                        dp[i][j] = cost;
                    }
                }
            }
        }

        return dp[1][n];
    }
};
```
