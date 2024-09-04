# 33. Search in Rotated Sorted Array

### 题目描述

给定一个升列排序后旋转了的数组，和目标值target。检查target是否存在于数组中。

### 题目解析

两次二分查找，第一次找旋转点，第二次根据target选择区间，再二分查找目标值是否存在。

### 代码实现

###### c++

```c++
class Solution {
public:
    int search(vector<int>& nums, int target) {
        int l = 0, r = nums.size() - 1;
        while (l < r)
        {
            int mid = l + (r - l) / 2 + 1;
            if (nums[mid] >= nums[0]) l = mid;
            else r = mid - 1;
        }

        if (target >= nums[0]) l = 0;
        else l = l + 1, r = nums.size() - 1;
        while (l < r)
        {
            int mid = l + (r - l) / 2 + 1;
            if (nums[mid] <= target) l = mid;
            else r = mid - 1;
        }
        return nums[r] == target ? r : -1;
    }
};
```

The following solution seems to be more elegant:

```c++
// Credit to: https://leetcode.com/problems/search-in-rotated-sorted-array/solutions/5378208/video-find-a-sorted-part-in-ascending-order/

class Solution {
public:
    int search(vector<int>& nums, int target) {
        int left = 0;
        int right = nums.size() - 1;

        while (left <= right) {
            int mid = (left + right) / 2;

            if (nums[mid] == target) {
                return mid;
            } else if (nums[mid] >= nums[left]) {
                if (nums[left] <= target && target <= nums[mid]) {
                    right = mid - 1;
                } else {
                    left = mid + 1;
                }
            } else {
                if (nums[mid] <= target && target <= nums[right]) {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
        }

        return -1;        
    }
};
```