# 154. Find Minimum in Rotated Sorted Array II

### Description

Suppose an array of length n sorted in ascending order is rotated between 1 and n times. For example, the array nums = [0,1,4,4,5,6,7] might become:

- [4,5,6,7,0,1,4] if it was rotated 4 times.
- [0,1,4,4,5,6,7] if it was rotated 7 times.
Notice that rotating an array [a[0], a[1], a[2], ..., a[n-1]] 1 time results in the array [a[n-1], a[0], a[1], a[2], ..., a[n-2]].

Given the sorted rotated array nums that may contain duplicates, return the minimum element of this array.

You must decrease the overall operation steps as much as possible.

### Solution

依然可以排序后返回第一个，但不妨考虑一下二分查找的方式。

定义左端点Low，右端点high，中间值pivot。由于数组旋转，我们有nums[low] >= nums[high]。
- 如果pivot小于high，则pivot处于右半部分数组，旋转点一定在pivot左边，因此可以放弃pivot到右端点的部分。
- 如果pivot大于high，则pivot处于左半部分数组，旋转点一定在pivot右边，因此可以放弃pivot到左端点的部分。
- 如果pivot等于high，则不确定pivot处于哪一边，但至少可以放弃单个点。

### Implementation

###### c++

```c++
class Solution {
public:
    int findMin(vector<int>& nums) {
        int low = 0, high = nums.size() - 1;
        while (low < high) {
            int pivot = low + (high - low) / 2;
            if (nums[pivot] > nums[high]) {
                low = pivot + 1;
            } else if (nums[pivot] < nums[high]) {
                high = pivot;
            } else {
                high -= 1;
            }
        }
        return nums[low];
    }
};
```
