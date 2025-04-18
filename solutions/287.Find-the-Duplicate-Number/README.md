# 287. Find the Duplicate Number

### Description

Given an array of integers nums containing n + 1 integers where each integer is in the range [1, n] inclusive.

There is only one repeated number in nums, return this repeated number.

You must solve the problem without modifying the array nums and using only constant extra space.

### Example

###### Example I:

```
Input: nums = [1,3,4,2,2]
Output: 2
```

###### Example II:

```
Input: nums = [3,1,3,4,2]
Output: 3
```
###### Example III:

```
Input: nums = [3,3,3,3,3]
Output: 3
```

### Solution

考虑到数组中元素的范围和数组本身的长度，我们可以可以将以当前数字为下标的数组中的值变为负值，当遍历的过程中发现自己指向的是负值时，便代表这个数字曾经存在过。

```c++
class Solution {
public:
    int findDuplicate(vector<int>& nums) {
        for (int i = 0; i < nums.size(); i++) {
            int temp = abs(nums[i]);
            if (nums[temp] < 0) return temp;
            else nums[temp] = -nums[temp];
        }
        return 0;
    }
};
```
