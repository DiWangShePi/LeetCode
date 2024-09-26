# 55. Jump Game

### 题目描述

给定一个整数数组nums。您最初位于数组的第一个索引处，数组中的每个元素表示在该位置的最大跳转长度。如果能到达最后一个索引返回true，否则返回false。

**示例：**

```
Input: nums = [2,3,1,1,4]
Output: true
Explanation: Jump 1 step from index 0 to 1, then 3 steps to the last index.
```

```
Input: nums = [3,2,1,0,4]
Output: false
Explanation: You will always arrive at index 3 no matter what. Its maximum jump length is 0, which makes it impossible to reach the last index.
```

### 题目解析

定义数组canVisited指示输入数组的每个数字是否能抵达终点，初始化最后一个值为True。
从后往前遍历数组，若当前元素的步数范围内有一个值为true，则该元素同样为true。
返回第一个值是否为True。

但这是一个开销很大的算法。我们也可以正向遍历，尝试简单一些的算法。
我们定义cost值初始化为0，每一次指针移动使cost值减1。cost值为当前cost和nums[i]的最大值。
若遍历过程中cost值小于0，即不能抵达。若遍历完成，则可以抵达。

### 代码实现

###### c++

```c++
class Solution {
public:
    bool canJump(vector<int>& nums) {
        int gas = 0;
        for (int i = 0; i < nums.size(); i++) {
            if (gas < 0) return false;
            gas = gas > nums[i] ? gas : nums[i];
            gas--;
        }
        return true;
    }
};
```