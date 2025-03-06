# 215. Kth Largest Element in an Array

### Description

Given an integer array nums and an integer k, return the kth largest element in the array.

Note that it is the kth largest element in the sorted order, not the kth distinct element.

Can you solve it without sorting?

### Solution

没有人可以在完全不涉及排序的情况下解决这道题，题目的要求只是不要用sort()函数。

用堆是一个自然的想法，但他的复杂度是O(NlogN)。而排序的复杂度也就是这个。

> Therefore...

### Implementation

###### c++

```c++
#include <algorithm>

class Solution {
public:
    int findKthLargest(vector<int>& nums, int k) {
        sort(nums.begin(), nums.end());
        return nums[nums.size() - k];
    }
};
```
