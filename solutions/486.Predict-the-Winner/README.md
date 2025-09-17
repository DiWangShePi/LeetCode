# 486. Predict the Winner

### Description

You are given an integer array nums. Two players are playing a game with this array: player 1 and player 2.

Player 1 and player 2 take turns, with player 1 starting first. Both players start the game with a score of 0. At each turn, the player takes one of the numbers from either end of the array (i.e., nums[0] or nums[nums.length - 1]) which reduces the size of the array by 1. The player adds the chosen number to their score. The game ends when there are no more elements in the array.

Return true if Player 1 can win the game. If the scores of both players are equal, then player 1 is still the winner, and you should also return true. You may assume that both players are playing optimally.

### Example 

###### Example I

> Input: nums = [1,5,2]
> Output: false
> Explanation: Initially, player 1 can choose between 1 and 2. 
> If he chooses 2 (or 1), then player 2 can choose from 1 (or 2) and 5. If player 2 chooses 5, then player 1 will be left with 1 (or 2). 
> So, final score of player 1 is 1 + 2 = 3, and player 2 is 5. 
> Hence, player 1 will never be the winner and you need to return false.

###### Example II

> Input: nums = [1,5,233,7]
> Output: true
> Explanation: Player 1 first chooses 1. Then player 2 has to choose between 5 and 7. No matter which number player 2 choose, player 1 can choose 233.
> Finally, player 1 has more score (234) than player 2 (12), so you need to return True representing player1 can win.

### Solution

先考虑暴力的做法：深度优先搜索，遍历所有可能性

- 特别的，优先计算数组的值，当某一方的值超过一半时，可以直接判定结果而进行剪枝
- 对于A来说，只要选择开头或者选择结束的任何一个选项能赢时，就可以判定为赢
- 对于B来说，同样的道理，但B赢了就是A输了。因此需要两个选项均为false时，才能判定A能赢

```c++
class Solution {
public:
    bool predictTheWinner(vector<int>& nums) {
        long long half = accumulate(nums.begin(), nums.end(), 0LL) / 2;
        return dfs(nums, 0, 0, half, true, 0, nums.size() - 1);
    }

private:
    bool dfs(vector<int>& nums, long long playerA, long long playerB, long long half, bool turn, int i, int j) {
        if (i > j) return playerA >= playerB;
        if (playerA > half) return true;
        if (playerB > half) return false;

        if (turn) {
            return dfs(nums, playerA + nums[i], playerB, half, false, i + 1, j) | dfs(nums, playerA + nums[j], playerB, half, false, i, j - 1);    
        } else {
            return dfs(nums, playerA, playerB + nums[i], half, true , i + 1, j) & dfs(nums, playerA, playerB + nums[j], half, true , i, j - 1);
        }
    }
};
```

更优的解法是采用动态规划，我们定义dp[i][j]为区间[i, j]内玩家之间能够达到的最大分差。分差为正即代表先手能赢。

显然，当i > j时，数组值没有意义。当i = j时，玩家只能选择这一个数字。当i < j时，玩家可以选择i和j两种可能，因此dp[i][j] = max(nums[i] - dp[i + 1][j], nums[j] - dp[i][j + 1])

注意这里是减去，因为下一轮是对手选择，所以我在当前能得到的最大分差是我当前能获得的值减去对手能达到的最大分差。

```c++
class Solution {
public:
    bool predictTheWinner(vector<int>& nums) {
        int n = nums.size();
        vector<vector<int>> dp(n, vector<int>(n, 0));
        for (int i = 0; i < n; i++) dp[i][i] = nums[i];

        for (int i = n - 2; i >= 0; i--) {
            for (int j = i + 1; j < n; j++) {
                dp[i][j] = max(nums[i] - dp[i + 1][j], nums[j] - dp[i][j - 1]);
            }
        }
        return dp[0][n - 1] >= 0;
    }
};
```

对于上述解法，我们可以进一步优化其空间复杂度。因为i是逆序遍历，j是正序遍历。我们可以用一维的数组保留上一轮的结果，即更像时需要用到的dp[i + 1][j]，而另一部分需要用到的dp[i][j - 1]则因为j是正序遍历的而已经被更新了。

```c++
class Solution {
public:
    bool PredictTheWinner(vector<int>& nums) {
        int n = nums.size();
        vector<int> dp(nums);  // 初始化为 dp[i][i] = nums[i]

        for (int i = n - 2; i >= 0; i--) {
            for (int j = i + 1; j < n; j++) {
                dp[j] = max(nums[i] - dp[j], nums[j] - dp[j - 1]);
            }
        }
        return dp[n - 1] >= 0;
    }
};
```
