# 643. Maximum Average Subarray I

**Tags:** Sliding Window

### Description

You are given an integer array nums consisting of n elements, and an integer k.

Find a contiguous subarray whose length is equal to k that has the maximum average value and return this value. Any answer with a calculation error less than 10-5 will be accepted.

### Example

###### Example I

> Input: nums = [1,12,-5,-6,50,3], k = 4
> Output: 12.75000
> Explanation: Maximum average is (12 - 5 - 6 + 50) / 4 = 51 / 4 = 12.75

###### Example II

> Input: nums = [5], k = 1
> Output: 5.00000

### Solution

滑动窗口，参见：https://leetcode.cn/discuss/post/3578981/ti-dan-hua-dong-chuang-kou-ding-chang-bu-rzz7/

```c++
class Solution {
public:
    double findMaxAverage(vector<int>& nums, int k) {
        double sum = 0, an = INT_MIN;
        int index = 0;
        while (index < nums.size()) {
            sum += nums[index];
            index++;
            if (index >= k) {
                an = max(an, sum / k);
                sum -= nums[index - k];
            }
        }
        return an;
    }
};
```
