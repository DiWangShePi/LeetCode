# 209. Minimum Size Subarray Sum

### Description

Given an array of positive integers nums and a positive integer target, return the minimal length of a subarray whose sum is greater than or equal to target. If there is no such subarray, return 0 instead.

### Solution

滑动窗口

### Implementation

###### c++

```c++
class Solution {
public:
    int minSubArrayLen(int s, vector<int>& nums) {
        if (nums.size() == 0) return 0;
        int ans = INT_MAX;
        int l = 0, r = 0;
        int sum = 0;
        while (r < nums.size()) {
            sum += nums[r];
            while (sum >= s) {
                ans = min(ans, r - l + 1);
                sum -= nums[l++];
            }
            r++;
        }
        return ans == INT_MAX ? 0 : ans;
    }
};
```
