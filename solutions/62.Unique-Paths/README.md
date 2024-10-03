# 62. Unique Paths

### 题目描述

在m x n的网格上有一个机器人。机器人最初位于左上角(即网格[0][0])。机器人尝试移动到右下角(即网格[m - 1][n - 1])。在任何时间点，机器人只能向下或向右移动。给定两个整数m和n，返回机器人到达右下角可能经过的唯一路径的个数。

我们确保生成的测试用例，使答案小于或等于2 * 109。

```
Input: m = 3, n = 7
Output: 28
```

```
Input: m = 3, n = 2
Output: 3
Explanation: From the top-left corner, there are a total of 3 ways to reach the bottom-right corner:
1. Right -> Down -> Down
2. Down -> Down -> Right
3. Down -> Right -> Down
```

### 题目解析

深度优先遍历，每一次往右走或者往下走，若能达到终点则答案加一。

简单直观的解法，可惜会爆栈。我们需要另一种思路。

考虑到机器人每一步只能往下或者往右走，这意味着，假设有N条路径走到当前格子，N_Left为走到当前格子左边的路径数量，N_Above为走到当前各自上方的路径数量，我们有：
N = N_Left + N_Above。

由此，我们可以建立状态转移矩阵，矩阵每个元素的值为该元素左边的值加上上面的值（不存在时即为0）。初始化矩阵(0,0)位置的值为1，状态转移结束后返回右下角的值即可。

### 代码实现

###### c++

```c++
class Solution {
public:
    int uniquePaths(int m, int n) {
        std::vector<int> aboveRow(n, 1);

        for (int row = 1; row < m; row++) {
            std::vector<int> currentRow(n, 1);
            for (int col = 1; col < n; col++) {
                currentRow[col] = currentRow[col - 1] + aboveRow[col];
            }
            aboveRow = currentRow;
        }

        return aboveRow[n - 1];        
    }
};
```