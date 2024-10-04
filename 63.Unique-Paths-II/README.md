# 63. Unique Paths II

### 题目描述

给你一个m × n的整数数组网格。有一个机器人最初位于左上角(即网格[0][0])。机器人尝试移动到右下角(即网格[m - 1][n - 1])。在任何时间点，机器人只能向下或向右移动。在网格中，障碍物和空间分别被标记为1或0。机器人所走的路径不能包含任何有障碍物的方块。返回机器人到达右下角的可能唯一路径的个数。生成测试用例，以便答案小于或等于2 * 109。

**示例：**

```
Input: obstacleGrid = [[0,0,0],[0,1,0],[0,0,0]]
Output: 2
Explanation: There is one obstacle in the middle of the 3x3 grid above.
There are two ways to reach the bottom-right corner:
1. Right -> Right -> Down -> Down
2. Down -> Down -> Right -> Right
```

```
Input: obstacleGrid = [[0,1],[0,0]]
Output: 1
```

### 题目解析

与上一题一样，不同的是当左边或者上面为砖块时，计入状态转移的该值为0。

### 代码实现

###### c++

```c++
class Solution {
public:
    int uniquePathsWithObstacles(vector<vector<int>>& obstacleGrid) {
        int m = obstacleGrid.size();
        int n = obstacleGrid[0].size();

        vector<vector<int>> result(m, vector(n, 1));
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                if (obstacleGrid[i][j] == 1) {
                    result[i][j] = 0;
                    continue;
                }
                if (i == 0 && j == 0) {
                    result[i][j] == 1;
                    continue;
                }

                int left = i > 0 ? result[i-1][j] : 0;
                int above = j > 0 ? result[i][j-1] : 0;
                result[i][j] = left + above;
            }
        }
        return obstacleGrid[m-1][n-1] == 1 ? 0 : result[m-1][n-1];
    }
};
```