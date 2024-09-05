# 34. Find First and Last Position of Element in Sorted Array

### 题目描述

给定一个非降序排列的数组，找到目标值target的第一个和最后一个。

如果目标值没有找到，则返回-1。

你需要使用O(logn)时间复杂度的算法。

**示例：**

```
Input: nums = [5,7,7,8,8,10], target = 8
Output: [3,4]
```

```
Input: nums = [5,7,7,8,8,10], target = 6
Output: [-1,-1]
```

```
Input: nums = [], target = 0
Output: [-1,-1]
```

### 题目解析

需要在O(logn)的时间复杂度内完成，即需要使用二分查找算法。

我们可以看到，找到第一个和最后一个的区别，即在于当前数字为target时指针的更新方式。
在寻找第一个时，当target与当前值相等，可能的值会出现在左边，即right=mid-1。
在寻找最后一个时，当target与当前值相等，可能的值会出现在右边，即left=mid+1。
但也有可能现在我们找到的值就是最左边或最右边的，因此我们留存一个值用于记录符合条件的下标，仅在当前值等于target的时候再次更新这个值。

### 代码实现

###### c++

```c++
class Solution {
public:
    vector<int> searchRange(vector<int>& nums, int target) {
        int left = search(nums, target, true);
        int right = search(nums, target, false);
        return {left, right};
    }

private:
    int search(vector<int>& nums, int target, bool firstOne) {
        int left = 0, right = nums.size() - 1, idx = -1;
        while (left < right) {
            int mid = left + (right - left) / 2;
            if (nums[mid] < target) {
                left = mid + 1;
            } else if (nums[mid] > target) {
                right = mid - 1;
            } else {
                idx = mid;
                if (firstOne) {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            }
        }
        return idx;
    }
};
```