# 283. Move Zeroes

### Description

Given an integer array nums, move all 0's to the end of it while maintaining the relative order of the non-zero elements.

Note that you must do this in-place without making a copy of the array.

### Example

###### Example I:

```
Input: nums = [0,1,0,3,12]
Output: [1,3,12,0,0]
```

###### Example II:

```
Input: nums = [0]
Output: [0]
```

### Solution

创建一个额外的数组，遍历nums的过程中将不为0的数字按顺序加入到其中，随后将该数组内容复制到nums中，后续的全部置为0。

```c++
class Solution {
public:
    void moveZeroes(vector<int>& nums) {
        vector<int> record;
        for (int i = 0; i < nums.size(); i++) if (nums[i] != 0) record.push_back(nums[i]);
        int i = 0;
        for ( ; i < record.size(); i++) nums[i] = record[i];
        while (i < nums.size()) nums[i++] = 0;
    }
};
```
