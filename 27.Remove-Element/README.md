# 27. Remove Element

### 题目描述

给定数组nums和要删除的数字val，删除数组中的所有val，返回数组中不等于val的元素的个数。

此外，你需要确保数组中的前K个数字为该K个不等于val的数字。你不需要将其进行排序。

**示例：**

```
Input: nums = [3,2,2,3], val = 3
Output: 2, nums = [2,2,_,_]
Explanation: Your function should return k = 2, with the first two elements of nums being 2.
It does not matter what you leave beyond the returned k (hence they are underscores).
```

### 题目解析

遍历数组，指针k指向数组末尾。每一轮检查当前元素是否等于val，若是则与指针k指向的元素交换，若不是则移动至下一个。

### 代码实现

```c++
class Solution {
public:
    int removeElement(vector<int>& nums, int val) {
        if (nums.size() == 0) {
            return 0;
        }

        int k = nums.size()-1;
        for (int i = 0; i < nums.size(); i++) {
            if (nums[i] == val) {
                while(nums[k] == val && k > i) {
                    k--;
                }

                if (k == i) {
                    break;
                }
                nums[i] = nums[k];
                nums[k] = val;
            }
        }
        return nums[k] == val ? k : k+1;
    }
};
```
