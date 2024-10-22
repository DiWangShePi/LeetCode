# 81. Search in Rotated Sorted Array II

### 题目描述

有一个按非递减顺序排序的整数数组 nums（不一定具有不同的值）。

在传递给您的函数之前，nums 会在未知的枢轴索引 k（0 <= k < nums.length）处旋转，使得生成的数组为 [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]]（0 索引）。例如，[0,1,2,4,4,4,5,6,6,7] 可能会在枢轴索引 5 处旋转并变为 [4,5,6,6,7,0,1,2,4,4]。

给定旋转后的数组 nums 和一个整数 target，如果 target 在 nums 中，则返回 true，如果它不在 nums 中，则返回 false。

您必须尽可能减少整体操作步骤。

**示例：**

```
Input: nums = [2,5,6,0,0,1,2], target = 0
Output: true
```

```
Input: nums = [2,5,6,0,0,1,2], target = 3
Output: false
```

### 题目解析

二分，第一步找旋转点，第二步找数值。

### 代码实现

###### c++

```c++
class Solution {
public:
    bool search(vector<int>& nums, int target) {
        int l = 0, r = nums.size() - 1;
        while ( l < r && nums[l] == nums[r]) r--;

        while (l <= r) {
            int mid = l + (r - l) / 2;
            if (nums[mid] >= nums[0]) l = mid + 1; 
            else r = mid - 1;
        }

        if (target > nums[0]) l = 0;
        else r = nums.size() - 1;

        while (l <= r) {
            int mid = l + (r - l) / 2;
            if (nums[mid] == target) return true;
            if (nums[mid] > target) r = mid - 1; 
            else l = mid + 1;
        }
        return false;
    }
};
```