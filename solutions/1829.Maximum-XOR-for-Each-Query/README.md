# 1829. Maximum XOR for Each Query

**Tags:** Bitwise Operation

### Description

You are given a sorted array nums of n non-negative integers and an integer maximumBit. You want to perform the following query n times:

1. Find a non-negative integer k < 2maximumBit such that nums[0] XOR nums[1] XOR ... XOR nums[nums.length-1] XOR k is maximized. k is the answer to the ith query.
2. Remove the last element from the current array nums.
Return an array answer, where answer[i] is the answer to the ith query.

### Example

###### Example I

> Input: nums = [0,1,1,3], maximumBit = 2
> Output: [0,3,2,3]
> Explanation: The queries are answered as follows:
> 1st query: nums = [0,1,1,3], k = 0 since 0 XOR 1 XOR 1 XOR 3 XOR 0 = 3.
> 2nd query: nums = [0,1,1], k = 3 since 0 XOR 1 XOR 1 XOR 3 = 3.
> 3rd query: nums = [0,1], k = 2 since 0 XOR 1 XOR 2 = 3.
> 4th query: nums = [0], k = 3 since 0 XOR 3 = 3.

###### Example II

> Input: nums = [2,3,4,7], maximumBit = 3
> Output: [5,2,6,5]
> Explanation: The queries are answered as follows:
> 1st query: nums = [2,3,4,7], k = 5 since 2 XOR 3 XOR 4 XOR 7 XOR 5 = 7.
> 2nd query: nums = [2,3,4], k = 2 since 2 XOR 3 XOR 4 XOR 2 = 7.
> 3rd query: nums = [2,3], k = 6 since 2 XOR 3 XOR 6 = 7.
> 4th query: nums = [2], k = 5 since 2 XOR 5 = 7.

###### Example III

> Input: nums = [0,1,2,2,5,7], maximumBit = 3
> Output: [4,3,6,4,6,7]

### Solution

获得每一个子数组的异或很简单，然后要做的是获取到使其异或最大的 k 值。这个的做法就是将异或结果中每一位的 0 变为 1，1 变为 0。

```c++
class Solution {
public:
    vector<int> getMaximumXor(vector<int>& nums, int maximumBit) {
        int xo = nums[0], n = nums.size();
        for (int i = 1; i < n; i++) xo ^= nums[i];

        int mask = 0;
        for (int i = 0; i < maximumBit; i++) mask |= (1 << i);

        vector<int> an(n, 0);
        an[0] = xo ^ mask;
        for (int i = n - 2; i > -1; i--) {
            xo ^= nums[i + 1];
            an[n - i - 1] = xo ^ mask;
        }
        return an;
    }
};
```
