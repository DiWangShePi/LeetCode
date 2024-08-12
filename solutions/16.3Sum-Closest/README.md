# 16. 3Sum Closest

### 题目描述

给定长度为n的整数数组`nums`和目标值`target`，在数组中寻找三个整数，使得三者之和最接近`target`。

返回三者之和

可以假定每个输入有且仅有一个解。

**示例：**

```
Input: nums = [-1,2,1,-4], target = 1
Output: 2
Explanation: The sum that is closest to the target is 2. (-1 + 2 + 1 = 2).
```

```
Input: nums = [0,0,0], target = 1
Output: 0
Explanation: The sum that is closest to the target is 0. (0 + 0 + 0 = 0).
```

### 题目解析

延用三数之和的思路，采用枚举加双指针。

首先将数组排序，遍历数组。每一轮遍历中，固定i，定义left为i+1，right为nums.size()-1。计算三数之和，若其与当前记录的target值最接近，则更新记录。若大于当前记录，则将right减小，若小于当前记录，则将left增大。

### 代码实现

###### c++
```c++
#include <vector>
#include <algorithm>

class Solution {
public:
    int threeSumClosest(vector<int>& nums, int target) {
        int answer = nums[0] + nums[1] + nums[2];
        sort(nums.begin(), nums.end());

        for(int i = 0; i < nums.size()-2; i++) {
            int left = i+1, right = nums.size()-1;

            while(left < right) {
                int currentAn = nums[i] + nums[left] + nums[right];
                if (currentAn == target) return target;

                if (abs(currentAn - target) < abs(answer - target)) answer = currentAn;
                if (currentAn > target) right--;
                else left++;
            }
        }

        return answer;
    }
};
```