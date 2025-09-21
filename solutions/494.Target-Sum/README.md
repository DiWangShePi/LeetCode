# 494. Target Sum

### Description

You are given an integer array nums and an integer target.

You want to build an expression out of nums by adding one of the symbols '+' and '-' before each integer in nums and then concatenate all the integers.

- For example, if nums = [2, 1], you can add a '+' before 2 and a '-' before 1 and concatenate them to build the expression "+2-1".
Return the number of different expressions that you can build, which evaluates to target.

### Example

###### Example I

> Input: nums = [1,1,1,1,1], target = 3
> Output: 5
> Explanation: There are 5 ways to assign symbols to make the sum of nums be target 3.
> -1 + 1 + 1 + 1 + 1 = 3
> +1 - 1 + 1 + 1 + 1 = 3
> +1 + 1 - 1 + 1 + 1 = 3
> +1 + 1 + 1 - 1 + 1 = 3
> +1 + 1 + 1 + 1 - 1 = 3

###### Example II

> Input: nums = [1], target = 1
> Output: 1

### Solution

当然，最经典的就是暴力枚举：

```c++
class Solution {
public:
    int findTargetSumWays(vector<int>& nums, int target) {
        int an = 0;
        dfs(nums, 0, 0, target, an);
        return an;
    }

private:
    void dfs(vector<int>& nums, int index, int currentSum, int target, int& an) {
        if (index == nums.size()) {
            if (currentSum == target) an++;
            return;
        }

        dfs(nums, index + 1, currentSum + nums[index], target, an);
        dfs(nums, index + 1, currentSum - nums[index], target, an);
        return;
    }
};
```

接下来，我们考虑动态规划的方式。

假定给定的数组中，前面需要加上负号的元素之和为neg，所有元素之和为sum，我们有 (sum - neg) - neg = target。
即 neg = (sum - target) / 2。

这个问题便可以被转换为：在数组中选择元素构成和为neg的个数有多少种。

```c++
class Solution {
public:
    int findTargetSumWays(vector<int>& nums, int target) {
        int sum = accumulate(nums.begin(), nums.end(), 0);
        if (sum < target || (sum - target) % 2 != 0) return 0;

        int neg = (sum - target) / 2, n = nums.size();
        vector<vector<int>> dp(n + 1, vector<int>(neg + 1, 0));
        dp[0][0] = 1;

        for (int i = 1; i <= n; i++) {
            int val = nums[i - 1];
            for (int j = 0; j <= neg; j++) {
                dp[i][j] = dp[i - 1][j];
                if (j >= val) dp[i][j] += dp[i - 1][j - val];
            }
        }
        return dp[n][neg];
    }
};
```

注意到每一轮的状态仅与上一轮有关，我们可以借此压缩

```c++
class Solution {
public:
    int findTargetSumWays(vector<int>& nums, int target) {
        int sum = accumulate(nums.begin(), nums.end(), 0);
        if (sum < target || (sum - target) % 2 != 0) return 0;

        int neg = (sum - target) / 2, n = nums.size();
        vector<int> dp(neg + 1, 0);
        dp[0] = 1;

        for (int val : nums) {
            for (int j = neg; j >= val; j--) {
                dp[j] += dp[j - val];
            }
        }
        return dp[neg];
    }
};
```
