# 581. Shortest Unsorted Continuous Subarray

**Tags:**

### Description

Given an integer array nums, you need to find one continuous subarray such that if you only sort this subarray in non-decreasing order, then the whole array will be sorted in non-decreasing order.

Return the shortest such subarray and output its length.

### Example

###### Example I

> Input: nums = [2,6,4,8,10,9,15]
> Output: 5
> Explanation: You need to sort [6, 4, 8, 10, 9] in ascending order to make the whole array sorted in ascending order.

###### Example II

> Input: nums = [1,2,3,4]
> Output: 0

###### Example III

> Input: nums = [1]
> Output: 0

### Solution

复制数组排序一遍，和原数组对不上的那一段就是最短的排序后整体有序的。

```c++
class Solution {
public:
    int findUnsortedSubarray(vector<int>& nums) {
        if (nums.size() == 1) return 0;

        vector<int> t = nums;
        sort(t.begin(), t.end());

        int l = nums.size() - 1, r = 0;
        for (int i = 0; i < nums.size(); i++) {
            if (nums[i] != t[i]) {
                l = i;
                break;
            }
        }
        for (int i = nums.size() - 1; i > -1; i--) {
            if (nums[i] != t[i]) {
                r = i;
                break;
            }
        }
        return r < l ? 0 : r - l + 1;
    }
};
```
