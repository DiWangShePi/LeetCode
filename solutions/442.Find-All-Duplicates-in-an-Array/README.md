# 442. Find All Duplicates in an Array

### Description

Given an integer array nums of length n where all the integers of nums are in the range [1, n] and each integer appears at most twice, return an array of all the integers that appears twice.

You must write an algorithm that runs in O(n) time and uses only constant auxiliary space, excluding the space needed to store the output

### Example

###### Example I

> Input: nums = [4,3,2,7,8,2,3,1]
> Output: [2,3]

###### Example II

> Input: nums = [1,1,2]
> Output: [1]

###### Example III

> Input: nums = [1]
> Output: []

### Solution

这道题其实很简单，略有一点难度的在于要求常数的额外空间，这个要求也是之前出现过的。

```c++
class Solution {
public:
    vector<int> findDuplicates(vector<int>& nums) {
        vector<int> an;
        for (int i = 0; i < nums.size(); i++) {
            int c = nums[i] < 0 ? -nums[i] : nums[i];
            if (nums[c - 1] < 0) an.push_back(c);
            else nums[c - 1] = -nums[c - 1];
        }
        return an;
    }
};
```