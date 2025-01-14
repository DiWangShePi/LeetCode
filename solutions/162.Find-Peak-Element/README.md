# 162. Find Peak Element

### Description

A peak element is an element that is strictly greater than its neighbors.

Given a 0-indexed integer array nums, find a peak element, and return its index. If the array contains multiple peaks, return the index to any of the peaks.

You may imagine that nums[-1] = nums[n] = -∞. In other words, an element is always considered to be strictly greater than a neighbor that is outside the array.

You must write an algorithm that runs in O(log n) time.

### Solution

题目最后一句给出的要求具有误导性，其实遍历一遍也是可以解决的。

但我们不妨再尝试思考一个符合要求的解法。
我们可以将数组假象为一个波段，我们要找的是这个波段的一个局部高点。
对于任意一个点，以从左向右的视角来看，它要么在上坡，要么在下坡。要么在坡顶。

我们可以通过一个点和其临近点的大小关系，判断这个点处于上坡还是下坡，以此进行二分查找即可。

### Implementation

###### c++

```c++
class Solution {
public:
    int findPeakElement(vector<int>& nums) {
        if (nums.size() == 1) return 0;

        if (nums[0] > nums[1]) return 0;
        for (int i = 1; i < nums.size() - 1; i++) {
            if (nums[i] > nums[i - 1] && nums[i] > nums[i + 1]) return i;
        }
        if (nums[nums.size() - 1] > nums[nums.size() - 2]) return nums.size() - 1;

        return 0;
    }
};
```
