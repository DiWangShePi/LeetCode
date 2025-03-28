# 238. Product of Array Except Self

### Description

Given an integer array nums, return an array answer such that answer[i] is equal to the product of all the elements of nums except nums[i].

The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.

You must write an algorithm that runs in O(n) time and without using the division operation.

### Solution

遍历一遍获得所有元素的乘积，然后再遍历一遍，每个元素的目标值为总乘积除以当前元素的值。

> 这显然是要求中要ban掉的解法

我们可以初始化两个数组，L和R。L代表当前元素左边所有元素乘积之和，R代表当前元素右边左右元素乘积之和。初始化这两个数组需要两次遍历。随后即可计算结果。

但我们进一步发现，获得了L之后，我们在下一次遍历的过程中获得的乘积已经可以让我们知道答案了，因此不需要再创建R数组了。

### Implementation

###### c++

```c++
class Solution {
public:
    vector<int> productExceptSelf(vector<int>& nums) {
        vector<int> answer(nums.size(), 1);
        int lp = 1, rp = 1;
        int l = 0, r = nums.size() - 1;
        while (l < nums.size()) {
            answer[l] *= lp;
            answer[r] *= rp;
            lp *= nums[l++];
            rp *= nums[r--];
        }
        return answer;
    }
};
```
