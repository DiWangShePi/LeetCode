# 1658. Minimum Operations to Reduce X to Zero

**Tags:** Sliding Window

### Description

You are given an integer array nums and an integer x. In one operation, you can either remove the leftmost or the rightmost element from the array nums and subtract its value from x. Note that this modifies the array for future operations.

Return the minimum number of operations to reduce x to exactly 0 if it is possible, otherwise, return -1.

### Example

###### Example I

> Input: nums = [1,1,4,2,3], x = 5
> Output: 2
> Explanation: The optimal solution is to remove the last two elements to reduce x to zero.

###### Example II

> Input: nums = [5,6,7,8,9], x = 4
> Output: -1

###### Example III

> Input: nums = [3,2,20,1,1,3], x = 10
> Output: 5
> Explanation: The optimal solution is to remove the last three elements and the first two elements (5 operations in total) to reduce x to zero.

### Solution

维护一个滑动窗口，当窗口内的元素值等于数组值之和减去x时，记录窗口的长度。答案即为数组总长度减去最长的子数组。

```c++
class Solution {
public:
    int minOperations(vector<int>& nums, int x) {
        int target = accumulate(nums.begin(), nums.end(), 0) - x;
        int n = nums.size();
        if (target < 0) return -1;
        if (target == 0) return n;

        int len = 0, l = 0, cur = 0;
        for (int i = 0; i < n; i++) {
            cur += nums[i];
            while (cur > target) cur -= nums[l++];

            if (cur == target) len = max(len, i - l + 1);
        }
        return len == 0 ? -1 : n - len;
    }
};
```
