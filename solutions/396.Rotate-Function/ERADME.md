# 396. Rotate Function

### Description

You are given an integer array nums of length n.

Assume arrk to be an array obtained by rotating nums by k positions clock-wise. We define the rotation function F on nums as follow:

- F(k) = 0 * arrk[0] + 1 * arrk[1] + ... + (n - 1) * arrk[n - 1].
Return the maximum value of F(0), F(1), ..., F(n-1).

The test cases are generated so that the answer fits in a 32-bit integer.

### Example 

###### Example I

> Input: nums = [4,3,2,6]
> Output: 26
> Explanation:
> F(0) = (0 * 4) + (1 * 3) + (2 * 2) + (3 * 6) = 0 + 3 + 4 + 18 = 25
> F(1) = (0 * 6) + (1 * 4) + (2 * 3) + (3 * 2) = 0 + 4 + 6 + 6 = 16
> F(2) = (0 * 2) + (1 * 6) + (2 * 4) + (3 * 3) = 0 + 6 + 8 + 9 = 23
> F(3) = (0 * 3) + (1 * 2) + (2 * 6) + (3 * 4) = 0 + 2 + 12 + 12 = 26
> So the maximum value of F(0), F(1), F(2), F(3) is F(3) = 26.

###### Example II

> Input: nums = [100]
> Output: 0

### Solution

我们发现，如果从1到n逐个遍历，我们发现F(i)可以由F(i - 1)加上一个固定的值（所有数字之和），再减去数组长度乘以一个特定的值后获得。

```c++
class Solution {
public:
    int maxRotateFunction(vector<int>& nums) {
        long round = 0, sum = 0, n = nums.size();
        for (int i = 0; i < n; i++) {
            round += i * nums[i];
            sum += nums[i];
        }

        long an = INT_MIN, current;
        for (int i = 0; i < n; i++) {
            current = round + sum - (n * nums[n - i - 1]);
            round = current;
            an = max(an, current);
        }
        return an;
    }
};
```
