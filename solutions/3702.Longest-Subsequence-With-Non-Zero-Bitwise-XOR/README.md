# 3702. Longest Subsequence With Non-Zero Bitwise XOR

**Tags:** Bitwise Operation

### Description

You are given an integer array nums.

Return the length of the longest subsequence in nums whose bitwise XOR is non-zero. If no such subsequence exists, return 0.

### Example

###### Example I

> Input: nums = [1,2,3]
> Output: 2
> Explanation:
> One longest subsequence is [2, 3]. The bitwise XOR is computed as 2 XOR 3 = 1, which is non-zero.

###### Example II

> Input: nums = [2,3,4]
> Output: 3
> Explanation:
> The longest subsequence is [2, 3, 4]. The bitwise XOR is computed as 2 XOR 3 XOR 4 = 5, which is non-zero.

### Solution

先对所有的数字做异或，检查结果是否非零。

如果是，那么就可以返回数组的长度了。如果不是，那么检查是否存在非零元素。如果存在，那么答案为 n - 1，如果不存在（即全是0），那么答案为 0。

```c++
class Solution {
public:
    int longestSubsequence(vector<int>& nums) {
        int xo = nums[0], n = nums.size();
        for (int i = 1; i < n; i++) xo ^= nums[i];
        if (xo != 0) return n;
        
        int count = 0;
        for (int& num : nums) {
            if (num == 0) count++;
        }
        return count == n ? 0 : n - 1;
    }
};
```
