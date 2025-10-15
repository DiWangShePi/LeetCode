# 523. Continuous Subarray Sum

**Tags:** Hash Table, Prefix Sum

### Description

Given an integer array nums and an integer k, return true if nums has a good subarray or false otherwise.

A good subarray is a subarray where:

- its length is at least two, and
- the sum of the elements of the subarray is a multiple of k.

Note that:

- A subarray is a contiguous part of the array.
- An integer x is a multiple of k if there exists an integer n such that x = n * k. 0 is always a multiple of k.

### Example 

###### Example I

> Input: nums = [23,2,4,6,7], k = 6
> Output: true
> Explanation: [2, 4] is a continuous subarray of size 2 whose elements sum up to 6.

###### Example II

> Input: nums = [23,2,6,4,7], k = 6
> Output: true
> Explanation: [23, 2, 6, 4, 7] is an continuous subarray of size 5 whose elements sum up to 42.
> 42 is a multiple of 6 because 42 = 7 * 6 and 7 is an integer.

###### Example III

> Input: nums = [23,2,6,4,7], k = 13
> Output: false

### Solution

一个略显暴力的方式是：计算给定数组的前缀和，然后枚举可能的子序列，计算子序列和来检查是否满足条件。

这一解法是O(n^2)的时间复杂度。

```c++
class Solution {
public:
    bool checkSubarraySum(vector<int>& nums, int k) {
        if (nums.size() < 2) return false;

        vector<long long> sums{0};
        for (int i = 0; i < nums.size(); i++) {
            sums.push_back(sums.back() + nums[i]);
        }

        for (int i = 0; i < sums.size(); i++) {
            for (int j = i + 2; j < sums.size(); j++) {
                if ((sums[j] - sums[i]) % k == 0) return true;
            }
        }
        return false;
    }
};
```

一个更优雅一些的解法是：我们不再枚举可能的子序列组合，转而计算前缀和模K的余数。当两个前缀和的余数相等时，即代表中间的子序列和可以被K整除。

```c++
class Solution {
public:
    bool checkSubarraySum(vector<int>& nums, int k) {
        if (nums.size() < 2) return false;

        unordered_map<int, int> remainderMap;
        remainderMap[0] = -1; 
        
        int prefixSum = 0;
        for (int i = 0; i < nums.size(); i++) {
            prefixSum += nums[i];
            int remainder = prefixSum % k;
            
            if (remainderMap.find(remainder) != remainderMap.end()) {
                if (i - remainderMap[remainder] >= 2) {
                    return true;
                }
            } else {
                remainderMap[remainder] = i;
            }
        }
        return false;
    }
};
```
