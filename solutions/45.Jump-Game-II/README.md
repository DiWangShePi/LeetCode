# 45. Jump Game II

### 题目解析

给定一个下标从0开始的列表nums，长度为n。初始位置在0处。

每一个元素代表从当前位置所能走的最长距离。换句话说，如果你在nums[i]处，
你可以走到的最长位置nums[i+j]符合以下条件：
- 0 <= j <= nums[i]
- i+j < n

返回走到nums[n-1]的最小步数。

**示例：**

```
Input: nums = [2,3,1,1,4]
Output: 2
Explanation: The minimum number of jumps to reach the last index is 2. Jump 1 step from index 0 to 1, then 3 steps to the last index.
```

### 题目解析

定义元素的jump值为当前元素到最终位置的最小步数，数组最后一位元素的jump值即为0。
从数组的最后一位元素开始，从后向前遍历。每一位元素的jump值为当前元素所能涉及的所有
元素中jump值最小的+1。

遍历结束后返回nums[0]的jump值。

这样的解法会带来O(n)的空间复杂度和较高的时间复杂度。

我们不妨换一个视角，从开始的位置向后寻找。每次我们记录两个值near和far，代表上一步之后，我们能到达
的最近和最远距离，检查这个范围之内的所有数字，从而更新新的最远距离。新的最近距离即为当前的最近距离+1
（因为所有的位置均可以到达）。每更新一次范围，即代表多跳跃了一次。

### 代码实现

###### c++

```c++
#include <vector>
#include <algorithm>

class Solution {
public:
    int jump(std::vector<int>& nums) {
        int jumps = 0;
        int farthest = 0; 
        int currentEnd = 0; 

        for (int i = 0; i < nums.size() - 1; i++) {
            farthest = std::max(farthest, i + nums[i]); 
            if (i == currentEnd) { 
                jumps++; 
                currentEnd = farthest; 
            }
        }
        
        return jumps;
    }
};

```