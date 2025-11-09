# 560. Subarray Sum Equals K

### Description

Given an array of integers nums and an integer k, return the total number of subarrays whose sum equals to k.

A subarray is a contiguous non-empty sequence of elements within an array.

### Example

###### Example I

> Input: nums = [1,1,1], k = 2
> Output: 2

###### Example II

> Input: nums = [1,2,3], k = 3
> Output: 2

### Solution

我想用滑动窗口，但这需要数组具有单调的性质。这一题中数组元素可以是负数，因而不能这样处理。

我们要用前缀和与字典来解决这一问题。

```c++
class Solution {
public:
    int subarraySum(vector<int>& nums, int k) {
        unordered_map<int, int> dict;
        dict[0] = 1;
        int count = 0, prefix = 0;
        for (int num : nums) {
            prefix += num;
            if (dict.count(prefix - k) != 0) count += dict[prefix - k];
            dict[prefix]++;
        }
        return count;
    }
};
```