# 3866. First Unique Even Element

### Description

You are given an integer array nums.

Return an integer denoting the first even integer (earliest by array index) that appears exactly once in nums. If no such integer exists, return -1.

An integer x is considered even if it is divisible by 2.

### Example

###### Example I

> Input: nums = [3,4,2,5,4,6]
> Output: 2
> Explanation:
> Both 2 and 6 are even and they appear exactly once. Since 2 occurs first in the array, the answer is 2.

###### Example II

> Input: nums = [4,4]
> Output: -1
> Explanation:
> No even integer appears exactly once, so return -1.

### Solution

先遍历一遍统计数目，再遍历一遍找第一个为偶数且后面不再出现的。

```c++
class Solution {
public:
    int firstUniqueEven(vector<int>& nums) {
        unordered_map<int, int> dict;
        for (int& num : nums) dict[num]++;

        for (int& num : nums) {
            if (num % 2 == 0 && dict[num] == 1) return num;
        }
        return -1;
    }
};
```
