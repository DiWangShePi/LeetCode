# 674. Longest Continuous Increasing Subsequence

**Tags:** Sliding Window

### Description

Given an unsorted array of integers nums, return the length of the longest continuous increasing subsequence (i.e. subarray). The subsequence must be strictly increasing.

A continuous increasing subsequence is defined by two indices l and r (l < r) such that it is [nums[l], nums[l + 1], ..., nums[r - 1], nums[r]] and for each l <= i < r, nums[i] < nums[i + 1].

### Example

###### Example I

> Input: nums = [1,3,5,4,7]
> Output: 3
> Explanation: The longest continuous increasing subsequence is [1,3,5] with length 3.
> Even though [1,3,5,7] is an increasing subsequence, it is not continuous as elements 5 and 7 are separated by element 4.

###### Example II

> Input: nums = [2,2,2,2,2]
> Output: 1
> Explanation: The longest continuous increasing subsequence is [2] with length 1. Note that it must be strictly increasing.

### Solution

滑动窗口滑一遍就好了。

```c++
class Solution {
public:
    int findLengthOfLCIS(vector<int>& nums) {
        int ans = 1, l = 0;
        for (int i = 1; i < nums.size(); i++) {
            if (nums[i] <= nums[i - 1]) l = i;
            ans = max(ans, i - l + 1);
        }
        return ans;
    }
};
```
