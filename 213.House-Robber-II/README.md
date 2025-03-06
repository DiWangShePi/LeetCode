# 213. House Robber II

### Description

You are a professional robber planning to rob houses along a street. Each house has a certain amount of money stashed. All houses at this place are arranged in a circle. That means the first house is the neighbor of the last one. Meanwhile, adjacent houses have a security system connected, and it will automatically contact the police if two adjacent houses were broken into on the same night.

Given an integer array nums representing the amount of money of each house, return the maximum amount of money you can rob tonight without alerting the police.

### Solution

在前面一次抢劫的基础上，拓展为考虑两种情况：
1. 抢第一间，不抢最后一间。
2. 抢最后一间，不抢第一间。
在这两种情况内，就可以沿用此前的方式了。

### Implementation

###### c++

```c++
class Solution {
public:
    int robLinear(vector<int>& nums, int start, int end) {
        if (start == end) return nums[start];

        int prev2 = 0, prev1 = nums[start];
        for (int i = start + 1; i <= end; i++) {
            int curr = max(prev1, prev2 + nums[i]);
            prev2 = prev1;
            prev1 = curr;
        }
        return prev1;
    }

    int rob(vector<int>& nums) {
        int n = nums.size();
        if (n == 1) return nums[0];
        if (n == 2) return max(nums[0], nums[1]);

        return max(robLinear(nums, 0, n - 2), robLinear(nums, 1, n - 1));
    }
};
```