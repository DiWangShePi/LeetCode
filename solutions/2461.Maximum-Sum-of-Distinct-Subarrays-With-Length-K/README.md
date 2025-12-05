# 2461. Maximum Sum of Distinct Subarrays With Length K

**Tags:** Sliding Window, Dict

### Description

You are given an integer array nums and an integer k. Find the maximum subarray sum of all the subarrays of nums that meet the following conditions:

- The length of the subarray is k, and
- All the elements of the subarray are distinct.
Return the maximum subarray sum of all the subarrays that meet the conditions. If no subarray meets the conditions, return 0.

A subarray is a contiguous non-empty sequence of elements within an array.

### Example

###### Example I

> Input: nums = [1,5,4,2,9,9,9], k = 3
> Output: 15
> Explanation: The subarrays of nums with length 3 are:
> - [1,5,4] which meets the requirements and has a sum of 10.
> - [5,4,2] which meets the requirements and has a sum of 11.
> - [4,2,9] which meets the requirements and has a sum of 15.
> - [2,9,9] which does not meet the requirements because the element 9 is repeated.
> - [9,9,9] which does not meet the requirements because the element 9 is repeated.
> We return 15 because it is the maximum subarray sum of all the subarrays that meet the conditions

###### Example II

> Input: nums = [4,4,4], k = 3
> Output: 0
> Explanation: The subarrays of nums with length 3 are:
> - [4,4,4] which does not meet the requirements because the element 4 is repeated.
> We return 0 because no subarrays meet the conditions.

### Solutiion

字典加滑动窗口，参见：https://leetcode.cn/discuss/post/3578981/ti-dan-hua-dong-chuang-kou-ding-chang-bu-rzz7/

```c++
class Solution {
public:
    long long maximumSubarraySum(vector<int>& nums, int k) {
        long long sum = 0, an = 0;
        unordered_map<int, int> dict;
        for (int i = 0; i < nums.size(); i++) {
            sum += nums[i];
            dict[nums[i]]++;

            if (i >= k - 1) {
                if (dict.size() == k) an = max(an, sum);

                sum -= nums[i - k + 1];
                dict[nums[i - k + 1]]--;
                if (dict[nums[i - k + 1]] == 0) dict.erase(nums[i - k + 1]);
            }
        }
        return an;
    }
};
```
