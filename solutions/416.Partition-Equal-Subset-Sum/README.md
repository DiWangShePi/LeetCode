# 416. Partition Equal Subset Sum

### Description

Given an integer array nums, return true if you can partition the array into two subsets such that the sum of the elements in both subsets is equal or false otherwise.

### Example

###### Example I

> Input: nums = [1,5,11,5]
> Output: true
> Explanation: The array can be partitioned as [1, 5, 5] and [11].

###### Example II

> Input: nums = [1,2,3,5]
> Output: false
> Explanation: The array cannot be partitioned into equal sum subsets.

### Solution

将问题转换为：在原数组中寻找一套数字组合，使得这套数字组合的和等于列表总和的一半。

随后我们采用动态规划来处理，先求和并计算和的一半是多少（target），随后构建n乘以（target + 1）的动态规划数组。

dp[i][j]代表是否能在从0到i的数组中，寻找数字组合使得它的和为j。

对于给定数字nums[i]，如果j小于nums[i]，那么dp[i][j] = dp[i - 1][j]。如果j大于等于nums[i]，则dp[i - 1][j]或者dp[i - 1][j - nums[i]]任一成立，dp[i][j]成立。

```c++
class Solution {
public:
    bool canPartition(vector<int>& nums) {
        int n = nums.size();
        if (n < 2) return false;

        int sum = accumulate(nums.begin(), nums.end(), 0);
        if (sum & 1) return false;
        int maxNum = *max_element(nums.begin(), nums.end());
        int target = sum / 2;
        if (maxNum > target) return false;
        
        vector<vector<int>> dp(nums.size(), vector<int>(target + 1, 0));
        for (int i = 0; i < n; i++) dp[i][0] = true;
        dp[0][nums[0]] = true;

        for (int i = 1; i < n; i++) {
            int num = nums[i];
            for (int j = 1; j <= target; j++) {
                if (j >= num) dp[i][j] = dp[i - 1][j] | dp[i - 1][j - num];
                else dp[i][j] = dp[i - 1][j];
            }
        }
        return dp[n - 1][target];
    }
};
```
