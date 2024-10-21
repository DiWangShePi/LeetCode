# 80. Remove Duplicates from Sorted Array II

### 题目描述

给定一个按非递减顺序排序的整数数组 nums，就地删除一些重复项，使得每个唯一元素最多出现两次。元素的相对顺序应保持不变。

由于在某些语言中无法更改数组的长度，因此必须将结果放在数组 nums 的第一部分。更正式地说，如果在删除重复项后有 k 个元素，则 nums 的前 k 个元素应保存最终结果。前 k 个元素之后留下什么并不重要。

将最终结果放入 nums 的前 k 个位置后返回 k。

不要为另一个数组分配额外的空间。您必须通过使用 O(1) 额外内存就地修改输入数组来执行此操作。

**示例：**

```
Input: nums = [1,1,1,2,2,3]
Output: 5, nums = [1,1,2,2,3,_]
Explanation: Your function should return k = 5, with the first five elements of nums being 1, 1, 2, 2 and 3 respectively.
It does not matter what you leave beyond the returned k (hence they are underscores).
```

```
Input: nums = [0,0,1,1,1,1,2,3,3]
Output: 7, nums = [0,0,1,1,2,3,3,_,_]
Explanation: Your function should return k = 7, with the first seven elements of nums being 0, 0, 1, 1, 2, 3 and 3 respectively.
It does not matter what you leave beyond the returned k (hence they are underscores).
```

### 题目解析

两个指针，一个遍历数组，一个指向要修改的位置。一个额外的常数空间记录当前值出现的次数。

### 代码实现

###### c++

```c++
class Solution {
public:
    int removeDuplicates(vector<int>& nums) {
        if (nums.size() < 0) return 1;

        int pointer = 0;
        int fre = 1;
        for (int i = 1; i < nums.size(); i++) {
            if (nums[pointer] != nums[i]) {
                nums[++pointer] = nums[i];
                fre = 1;
            } else if (fre == 1 && nums[pointer] == nums[i]) {
                nums[++pointer] = nums[i];
                fre = 0;
            }
        }
        return pointer + 1;
    }
};
```