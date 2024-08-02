# 11. Container With Most Water

### 题目描述

给定一个长度为 n 的整数数组 height 。有 n 条垂线，第 i 条线的两个端点是 (i, 0) 和 (i, height[i]) 。

找出其中的两条线，使得它们与 x 轴共同构成的容器可以容纳最多的水。

返回容器可以储存的最大水量。

说明：你不能倾斜容器。

**示例：**

!()[question_11.jpg]
```
输入：[1,8,6,2,5,4,8,3,7]
输出：49 
解释：图中垂直线代表输入数组 [1,8,6,2,5,4,8,3,7]。在此情况下，容器能够容纳水（表示为蓝色部分）的最大值为 49。
```

```
输入：height = [1,1]
输出：1
```

### 题目解析

###### 暴力遍历
遍历数组，对于每一个元素$i$，计算该元素与其他所有元素组成的容器容量。记录最大值。

###### 双指针
创建指针head和tail，分别指向数组的头部和尾部，计算容量。不断更新其中一个指针的位置，更新并保留最大容量。更新方法为移动当前head和tail中较小的哪一个。

### 代码实现

###### c++

```c++
#include <vector>
#include <algorithm>

using namespace std;

class Solution {
public:
    int maxArea(vector<int>& height) {
        int left = 0, right = height.size() - 1;
        int maxArea = 0;

        auto getElementAtIndices = [&](int index1, int index2) -> int {
            int currentHeight = min(height[index1], height[index2]);
            return currentHeight * (index2 - index1);
        };

        while (left < right) {
            int area = getElementAtIndices(left, right);
            maxArea = max(maxArea, area);

            if (height[left] <= height[right]) {
                left ++;
            } else {
                right --;
            }
        }
        return maxArea;
    }
};
```

###### rust

```rust
use std::cmp;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left: usize = 0;
        let mut right: usize = height.len() - 1;

        let mut current_area = 0;
        let mut max_area = 0;

        while left < right {
            current_area = cmp::min(height[left], height[right]) * (right - left) as i32;
            max_area = cmp::max(current_area, max_area);
            
            if (height[left] <= height[right]) {
                left += 1;
            } else {
                right -= 1;
            }
        }

        max_area
    }
}
```