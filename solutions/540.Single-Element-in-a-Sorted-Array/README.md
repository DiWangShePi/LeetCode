# 540. Single Element in a Sorted Array

**Tags:** Binary Search

### Description

You are given a sorted array consisting of only integers where every element appears exactly twice, except for one element which appears exactly once.

Return the single element that appears only once.

Your solution must run in O(log n) time and O(1) space.

### Example 

###### Example I

> Input: nums = [1,1,2,3,3,4,4,8,8]
> Output: 2

###### Example II

> Input: nums = [3,3,7,7,10,11,11]
> Output: 10

### Solution

题目要求用O(logN)的解法，就是暗示要用二分查找。

对于每一次查找，检查 mid 是否为偶数，奇数时减一使其变为偶数。

接下来，对于[0, mid]这个数组。如果nums[mid] = nums[mid + 1]，就说明要找的元素在右边。反之即在左边。

```c++
class Solution {
public:
    int singleNonDuplicate(vector<int>& nums) {
        int left = 0, right = nums.size() - 1;
        
        while (left < right) {
            int mid = left + (right - left) / 2;
            if (mid % 2 == 1) mid--;
            
            if (nums[mid] == nums[mid + 1]) {
                left = mid + 2;
            } else {
                right = mid;
            }
        }
        return nums[left];
    }
};
```
