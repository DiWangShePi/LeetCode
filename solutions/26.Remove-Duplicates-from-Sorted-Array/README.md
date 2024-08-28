# 26. Remove Duplicates from Sorted Array

### 题目描述

给定一个排好序的数组（非减）。返回数组中独立元素的个数K，并确保数组的前K个元素为该K个独立元素。
元素之间的相对位置需要保持一致。

### 题目解析

循环数组，维护指针k。遍历数组，若当前数字不等于指针k指向的数字，则将指针向右移动一格，在
该位置填入当前数字。


遍历结束后返回数字k。

### 代码实现

###### c++

```c++
class Solution {
public:
    int removeDuplicates(vector<int>& nums) {
        int k = 0;
        for (int i = 0; i < nums.size(); i++) {
            if (nums[k] != nums[i]) {
                k++;
                nums[k] = nums[i];
            }
        }
        return k+1;
    }
};
```