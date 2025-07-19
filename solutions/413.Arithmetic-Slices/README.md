# 413. Arithmetic Slices

### Description

An integer array is called arithmetic if it consists of at least three elements and if the difference between any two consecutive elements is the same.

- For example, [1,3,5,7,9], [7,7,7,7], and [3,-1,-5,-9] are arithmetic sequences.
Given an integer array nums, return the number of arithmetic subarrays of nums.

A subarray is a contiguous subsequence of the array.

### Example 

###### Example I

> Input: nums = [1,2,3,4]
> Output: 3
> Explanation: We have 3 arithmetic slices in nums: [1, 2, 3], [2, 3, 4] and [1,2,3,4] itself.

###### Example II

> Input: nums = [1]
> Output: 0

### Solution

遍历一遍，计算所有连续的可构成要求数组的长度，随后计算这些子数组能构成的数组的数量之和

```c++
class Solution {
public:
    int numberOfArithmeticSlices(vector<int>& nums) {
        int n = nums.size();
        if (n < 3) return 0;

        vector<int> an; 
        int last_diff = nums[1] - nums[0];
        int len = 2; 
        
        for (int i = 2; i < n; i++) {
            if (nums[i] - nums[i - 1] == last_diff) {
                len++;
            } else {
                an.push_back(len);
                last_diff = nums[i] - nums[i - 1];
                len = 2;
            }
        }
        an.push_back(len);

        int result = 0;
        for (int a : an) {
            if (a < 3) continue;
            result += (a - 2) * (a - 1) / 2; 
        }
        return result;
    }
};
```
