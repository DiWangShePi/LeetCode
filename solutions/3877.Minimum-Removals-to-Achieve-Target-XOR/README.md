# 3877. Minimum Removals to Achieve Target XOR

**Tags:** Dynamic Programming

### Description

You are given an integer array nums and an integer target.

You may remove any number of elements from nums (possibly zero).

Return the minimum number of removals required so that the bitwise XOR of the remaining elements equals target. If it is impossible to achieve target, return -1.

The bitwise XOR of an empty array is 0.

### Example

###### Example I

> Input: nums = [1,2,3], target = 2
> Output: 1
> Explanation:
> Removing nums[1] = 2 leaves [nums[0], nums[2]] = [1, 3].
> The XOR of [1, 3] is 2, which equals target.
> It is not possible to achieve XOR = 2 in less than one removal, therefore the answer is 1.

###### Example II

> Input: nums = [2,4], target = 1
> Output: -1
> Explanation:
> It is impossible to remove elements to achieve target. Thus, the answer is -1.

###### Example III

> Input: nums = [7], target = 7
> Output: 0
> Explanation:
> The XOR of all elements is nums[0] = 7, which equals target. Thus, no removal is needed.

### Solution

移除的次数越小，等价于保留的个数越多。

```c++
class Solution {
public:
    int minRemovals(vector<int>& nums, int target) {
        int m = bit_width((uint32_t) ranges::max(nums));
        if ((1 << m) <= target) {
            return -1;
        }

        int n = nums.size();
        vector f(n + 1, vector<int>(1 << m, INT_MIN));
        f[0][0] = 0;

        for (int i = 0; i < n; i++) {
            int x = nums[i];
            for (int j = 0; j < (1 << m); j++) {
                f[i + 1][j] = max(f[i][j], f[i][j ^ x] + 1); // x 不选 or 选
            }
        }

        if (f[n][target] < 0) {
            return -1;
        }
        return nums.size() - f[n][target];
    }
};
```
