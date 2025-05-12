# 324. Wiggle Sort II

### Description

Given an integer array nums, reorder it such that nums[0] < nums[1] > nums[2] < nums[3]....

You may assume the input array always has a valid answer.

### Example

###### Example I

```
Input: nums = [1,5,1,1,6,4]
Output: [1,6,1,5,1,4]
Explanation: [1,4,1,5,1,6] is also accepted.
```

###### Example II

```
Input: nums = [1,3,2,2,3,1]
Output: [2,3,1,3,1,2]
```

### Solution

排序数组：首先将原始数组 nums 复制到一个临时数组 temp 中，并对 temp 进行排序。

分奇偶位置填充：根据数组长度的奇偶性，分别处理奇数位和偶数位的填充：

偶数长度：

- 从倒数第二个元素开始，向前每隔一个位置（即偶数索引）填充排序后的 temp 数组的前半部分（较小的数）。

- 然后从最后一个元素开始，向前每隔一个位置（即奇数索引）填充 temp 数组的后半部分（较大的数）。

奇数长度：

- 从最后一个元素开始，向前每隔一个位置（即偶数索引）填充排序后的 temp 数组的前半部分（较小的数）。

- 然后从倒数第二个元素开始，向前每隔一个位置（即奇数索引）填充 temp 数组的后半部分（较大的数）。

```c++
class Solution {
public:
    void wiggleSort(vector<int>& nums) {
        vector<int> temp;
        temp.assign(nums.begin(), nums.end());
        sort(temp.begin(), temp.end());

        int i = 0;
        if (nums.size() % 2 == 0) {
            for (int j = nums.size() - 2; j >= 0; j = j - 2) nums[j] = temp[i++];
            for (int j = nums.size() - 1; j >= 1; j = j - 2) nums[j] = temp[i++];
        } else {
            for (int j = nums.size() - 1; j >= 0; j = j - 2) nums[j] = temp[i++];
            for (int j = nums.size() - 2; j >= 1; j = j - 2) nums[j] = temp[i++];
        }
    }
};
```

> 但其实官解挺复杂的：https://leetcode.cn/problems/wiggle-sort-ii/solutions/1627858/bai-dong-pai-xu-ii-by-leetcode-solution-no0s
