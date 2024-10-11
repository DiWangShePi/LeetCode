# 70. Climbing Stairs

### 题目描述

你现在在爬楼梯，总共有N阶。每一次你可以走1步或者走两步，总共有多少种不同的方式可以爬上楼梯？

**示例：**

```
Input: n = 2
Output: 2
```

```
Input: n = 3
Output: 3
Explanation: There are three ways to climb to the top.
1. 1 step + 1 step + 1 step
2. 1 step + 2 steps
3. 2 steps + 1 step
```

### 题目解析

状态转移，初始值为1，每一步的值为上一步和上两步之和

### 代码实现

###### c++

```c++
class Solution {
public:
    int climbStairs(int n) {
        vector<int> result(n+1, 0);
        result[0] = 1;
        for (int i = 0; i < n; i++) {
            if (i + 1 <= n) result[i + 1] += result[i];
            if (i + 2 <= n) result[i + 2] += result[i];
        }
        return result[n];
    }
};
```
