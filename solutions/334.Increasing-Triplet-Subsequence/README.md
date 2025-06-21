# 334. Increasing Triplet Subsequence

### Description

Given an integer array nums, return true if there exists a triple of indices (i, j, k) such that i < j < k and nums[i] < nums[j] < nums[k]. If no such indices exists, return false.

### Example 

###### Example I

```
Input: nums = [1,2,3,4,5]
Output: true
Explanation: Any triplet where i < j < k is valid.
```

###### Example II

```
Input: nums = [5,4,3,2,1]
Output: false
Explanation: No triplet exists.
```

###### Example III

```
Input: nums = [2,1,5,0,4,6]
Output: true
Explanation: The triplet (3, 4, 5) is valid because nums[3] == 0 < nums[4] == 4 < nums[5] == 6.
```

### Solution

先遍历一遍数组，确定每个元素左边的最小值，再遍历一遍数组，找每个元素右边的最大值。最后遍历一遍数组，确定每个值左边的最小值和右边的最大值和自身的大小关系是否满足题目要求，满足即可。

```c++
class Solution {
public:
    bool increasingTriplet(vector<int>& nums) {
        int n = nums.size();
        if (n < 3) return false;

        // find left min number
        vector<int> leftMin(n, 0);
        leftMin[0] = nums[0];
        for (int i = 1; i < n; i++) leftMin[i] = min(leftMin[i - 1], nums[i]);

        // find right max number
        vector<int> rightMax(n, 0);
        rightMax[n - 1] = nums[n - 1];
        for (int i = n - 2; i > -1; i--) rightMax[i] = max(rightMax[i + 1], nums[i]);

        // find target
        for (int i = 0; i < n; i++){
            if (nums[i] > leftMin[i] && nums[i] < rightMax[i]) return true;
        }
        return false;
    }
};
```
