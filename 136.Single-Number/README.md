# 136. Single Number

### Description

Given a non-empty array of integers nums, every element appears twice except for one. Find that single one.

You must implement a solution with a linear runtime complexity and use only constant extra space.

### Solution

获得一个解法不难，有挑战的地方在于如何满足要求，即即在O(n)复杂度又在常数额外空间中。

为了解决这个问题，我们最终用到异或运算。

### Implementation

###### c++

```c++
class Solution {
public:
    int singleNumber(vector<int>& nums) {
        int ret = 0;
        for (auto e: nums) ret ^= e;
        return ret;
    }
};
```