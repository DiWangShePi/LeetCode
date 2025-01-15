# 164. Maximum Gap

### Description

Given an integer array nums, return the maximum difference between two successive elements in its sorted form. If the array contains less than two elements, return 0.

You must write an algorithm that runs in linear time and uses linear extra space.

### Solution

符合题目要求的是基数排序，但直接sort其实也可以。

### Implementation

###### c++

```c++
class Solution {
public:
    int maximumGap(vector<int>& nums) {
        if (nums.size() < 3) return 0;
        sort(nums.begin(), nums.end());
        int an = INT_MIN;
        for (int i = 1; i < nums.size(); i++) {
            an = max(an, nums[i] - nums[i - 1]);
        }
        return an;
    }
};
```
