# 189. Rotate Array

### Description

Given an integer array nums, rotate the array to the right by k steps, where k is non-negative.

### Solution

一个非常简单直接的想法是：将最后一个元素删除，插入到开头，如此重复k次。

这样，我们仅需要O(1)的额外空间，即可完成。但是，由于vector是动态数组，在开头插入元素需要O(n)的时间复杂度，因此这一解法开销会很大。

我们因而考虑第二种做法，将数组反转。

```
Input: nums = [1,2,3,4,5,6,7], k = 3
```

第一次完整反转，获得

```
[7,6,5,4,3,2,1]
```

再将前k个反转

```
[5,6,7,4,3,2,1]
```

再将后四个反转

```
[5,6,7,1,2,3,4]
```

### Implementation

###### c++

```c++
class Solution {
public:
    void rotate(vector<int>& nums, int k) {
        k = k % nums.size();
        reverse(nums.begin(), nums.end());
        reverse(nums.begin(), nums.begin() + k);
        reverse(nums.begin() + k, nums.end());
    }
};
```
