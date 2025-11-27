# 1004. Max Consecutive Ones III

**Tags:** Sliding Window

### Description

Given a binary array nums and an integer k, return the maximum number of consecutive 1's in the array if you can flip at most k 0's.

### Example

###### Example I

> Input: nums = [1,1,1,0,0,0,1,1,1,1,0], k = 2
> Output: 6
> Explanation: [1,1,1,0,0,1,1,1,1,1,1]
> Bolded numbers were flipped from 0 to 1. The longest subarray is underlined.

###### Example II

> Input: nums = [0,0,1,1,0,0,1,1,1,0,1,1,0,0,0,1,1,1,1], k = 3
> Output: 10
> Explanation: [0,0,1,1,1,1,1,1,1,1,1,1,0,0,0,1,1,1,1]
> Bolded numbers were flipped from 0 to 1. The longest subarray is underlined.

### Solution

滑动窗口，窗口内需要翻转的 0 大于期望值时移动左边界至满足条件。

```c++
class Solution {
public:
    int longestOnes(vector<int>& nums, int k) {
        int an = 0, cur = 0, l = 0, n = nums.size();
        for (int i = 0; i < n; i++) {
            if (nums[i] == 0) {
                cur++;
                while (cur > k) 
                    cur -= nums[l++] == 0;
            }

            an = max(an, i - l + 1);
        }
        return max(an, n - l);
    }
};
```
