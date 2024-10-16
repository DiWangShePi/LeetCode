# 75. Sort Colors

### 题目描述

给定一个数组 nums，其中包含 n 个红色、白色或蓝色的对象，对它们进行就地排序，使相同颜色的对象相邻，颜色顺序为红色、白色和蓝色。

我们将使用整数 0、1 和 2 分别表示红色、白色和蓝色。

您必须在不使用库的排序函数的情况下解决此问题。

**示例：**

```
Input: nums = [2,0,2,1,1,0]
Output: [0,0,1,1,2,2]
```

```
Input: nums = [2,0,1]
Output: [0,1,2]
```

### 题目解析

考虑到实际给出的要求，我们可以用一个排序算法解决这个问题。

但更进一步的，由于我们知道数组中的元素只有0，1，2。
并且我们知道他们所应该具有的顺序，唯一不知道的就是个数。

因此我们可以遍历一遍，找到0，1，2的个数，然后按个数填入数字即可。

### 代码实现

###### c++

```c++
class Solution {
public:
    void sortColors(vector<int>& nums) {
        int zero = 0, one = 0;
        for (int i = 0; i < nums.size(); i++) {
            switch(nums[i]) {
                case 0:
                    zero++;
                    break;
                case 1: 
                    one++;
                    break;
            }
        }
        for (int i = 0; i < nums.size(); i++) {
            if (i < zero) nums[i] = 0;
            else if (i < zero + one) nums[i] = 1;
            else nums[i] = 2;
        }
    }
};
```