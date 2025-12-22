# 992. Subarrays with K Different Integers

**Tags:** Sliding Window

### Description

Given an integer array nums and an integer k, return the number of good subarrays of nums.

A good array is an array where the number of different integers in that array is exactly k.

For example, [1,2,3,1,2] has 3 different integers: 1, 2, and 3.
A subarray is a contiguous part of an array.

### Example

###### Example I

> Input: nums = [1,2,1,2,3], k = 2
> Output: 7
> Explanation: Subarrays formed with exactly 2 different integers: [1,2], [2,1], [1,2], [2,3], [1,2,1], [2,1,2], [1,2,1,2]

###### Example II

> Input: nums = [1,2,1,3,4], k = 3
> Output: 3
> Explanation: Subarrays formed with exactly 3 different integers: [1,2,1,3], [2,1,3], [1,3,4].

### Solution

用字典记录出现的不同数字的个数，计算出现最多 K 个不同数字的子数组个数和出现最多 K - 1 个不同数字的子数组个数。这一计算过程可以使用滑动窗口。

```c++
class Solution {
public:
    int subarraysWithKDistinct(vector<int>& nums, int k) {
        return atMost(nums, k) - atMost(nums, k - 1);
    }

private:
    int atMost(vector<int>& nums, int k) {
        unordered_map<int, int> dict;
        int l = 0, an = 0;
        for (int i = 0; i < nums.size(); i++) {
            if (dict.count(nums[i]) == 0) dict[nums[i]] = 1;
            else dict[nums[i]]++;

            while (dict.size() > k) {
                dict[nums[l]]--;
                if (dict[nums[l]] == 0) dict.erase(nums[l]);
                l++;
            }

            an += i - l + 1;
        }
        return an;
    }
};
```
