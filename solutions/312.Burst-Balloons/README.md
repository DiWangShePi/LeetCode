# 312. Burst Balloons

### Description

You are given n balloons, indexed from 0 to n - 1. Each balloon is painted with a number on it represented by an array nums. You are asked to burst all the balloons.

If you burst the ith balloon, you will get nums[i - 1] * nums[i] * nums[i + 1] coins. If i - 1 or i + 1 goes out of bounds of the array, then treat it as if there is a balloon with a 1 painted on it.

Return the maximum coins you can collect by bursting the balloons wisely.

### Example 

###### Example I

```
Input: nums = [3,1,5,8]
Output: 167
Explanation:
nums = [3,1,5,8] --> [3,5,8] --> [3,8] --> [8] --> []
coins =  3*1*5    +   3*5*8   +  1*3*8  + 1*8*1 = 167
```

###### Example II

```
Input: nums = [1,5]
Output: 10
```

### Solution

每次戳一个气球后，剩下气球的“相邻关系”会动态改变，导致正向做难以确定当前戳破能带来的真实收益。一旦做了当前决策，就改变了之后每一步的“上下文环境”，让每次递归都必须动态更新相邻关系。这会非常复杂。而从“最后一个戳破”来考虑，可以固定左右边界，明确收益。因此，采用动态规划处理这一题的时候，我们定义dp[left][right]为左右边界为left,right时能获取的最大收益。随后枚举区间中的元素i为最后一个戳破的气球，考虑此时的收益。

由于i为最后一个戳破的气球，因此在戳破i时，气球之间的相邻关系为left,i,right，收益为nums[left] * nums[i] * nums[right]。而戳破i之前获取的收益为dp[left][i]和dp[i][right]。总和即为可用于当前dp[left][right]进行比较的更新值。

```c++
class Solution {
public:
    int maxCoins(vector<int>& nums) {
        vector<int> newNums{1};
        for (const int & num : nums) newNums.push_back(num);
        newNums.push_back(1);

        int n = newNums.size();
        vector<vector<int>> dp(n, vector<int>(n, 0));
        for (int length = 2; length < n; length++) {
            for (int left = 0; left + length < n; left++) {
                int right = left + length;

                for (int i = left + 1; i < right; i++) {
                    int coin = newNums[left] * newNums[i] * newNums[right];
                    dp[left][right] = max(dp[left][right], dp[left][i] + coin + dp[i][right]);
                }
            }
        }
        return dp[0][n - 1];
    }
};
```
