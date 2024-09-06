# 35. Search Insert Position

### 题目描述

给定一个排好序的数组和一个目标值。若数组中存在该值，则返回该值的下标。若不存在，则返回该值应该插入的位置，使得插入后的数组依然是升序的。

你的算法应当是O(logn)时间复杂度的。

### 题目解析

显然，应当用二分排序。

### 代码实现

###### c++

```c++
class Solution {
public:
    int searchInsert(vector<int>& nums, int target) {
        int l = 0, r = nums.size() - 1;
        while (l <= r) {
            int mid = l + (r - l) / 2;
            if (nums[mid] > target) {
              r = mid - 1;
            } else if (nums[mid] < target) {
              l = mid + 1;
            } else {
                return mid;
            }
        }
        return l;
    }
};
```