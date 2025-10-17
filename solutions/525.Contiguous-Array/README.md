# 525. Contiguous Array

**Tags:** Hash Table, Prefix Sum

### Description

Given a binary array nums, return the maximum length of a contiguous subarray with an equal number of 0 and 1.

### Example 

###### Example I

> Input: nums = [0,1]
> Output: 2
> Explanation: [0, 1] is the longest contiguous subarray with an equal number of 0 and 1.

###### Example II

> Input: nums = [0,1,0]
> Output: 2
> Explanation: [0, 1] (or [1, 0]) is a longest contiguous subarray with equal number of 0 and 1.

###### Example III

> Input: nums = [0,1,1,1,1,1,0,0,0]
> Output: 6
> Explanation: [1,1,1,0,0,0] is the longest contiguous subarray with equal number of 0 and 1.

### Solution

跟 LeetCode 523 一样，用前缀和和哈希表即可

```c++
class Solution {
public:
    int findMaxLength(vector<int>& nums) {
        int an = 0, t = 0, sum = 0;
        unordered_map<int, int> dict;
        dict[1] = -1;
        for (int i = 0; i < nums.size(); i++) {
            sum += nums[i];
            t = 2 * sum - i;
            if (dict.count(t) == 0) dict[t] = i;
            else {
                an = max(an, i - dict[t]);
            }
        }
        return an;
    }
};
```

### Related "Work"

1. 523. Continuous Subarray Sum