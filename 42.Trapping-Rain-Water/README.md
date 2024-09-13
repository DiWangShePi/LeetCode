# 42. Trapping Rain Water

### 题目描述

给定n个非负整数，表示海拔图，其中每个条的宽度为1，计算下雨后它可以捕获多少水。

**示例：**

![](./rainwatertrap.png)

```
Input: height = [0,1,0,2,1,0,1,3,2,1,2,1]
Output: 6
Explanation: The above elevation map (black section) is represented by array [0,1,0,2,1,0,1,3,2,1,2,1]. In this case, 6 units of rain water (blue section) are being trapped.
```

```
Input: height = [4,2,0,3,2,5]
Output: 9
```

### 题目解析

初始化变量左边界高度为数组第一个变量的值，初始化变量右边界高度为数组最后一个变量的值。
比较左边界高度和右边界高度，向中间移动较小的哪一个。
如果移动的是左边界，则比较当前值与记录下来的最高左边界高度。
若当前值大于等于记录下来的高度，则更新高度。若小于，则用记录值减去当前值，作为可保留的水。
若移动的是右边界则同理。

### 代码实现

###### c++

```c++
class Solution {
public:
    int trap(vector<int>& height) {
        int l_index = 0;
        int r_index = height.size() - 1;
        int l_height = height[l_index];
        int r_height = height[r_index];
        int water = 0;

        while (l_index < r_index) {
            if (l_height <= r_height) {
                l_index++;
            } else {
                r_index--;
            }

            l_height = l_height > height[l_index] ? l_height : height[l_index];
            r_height = r_height > height[r_index] ? r_height : height[r_index];

            water = l_height > height[l_index] ? water + (l_height - height[l_index]) : water;
            water = r_height > height[r_index] ? water + (r_height - height[r_index]) : water;
        }
        return water;
    }
};
```