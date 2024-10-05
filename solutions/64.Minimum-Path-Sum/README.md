# 64. Minimum Path Sum

### 题目描述

给定一个m × n的非负数网格，找到一条从左上到右下的路径，使路径上所有数字的总和最小。注意:您只能在任何时间点向下或向右移动。

**示例：**

```
Input: grid = [[1,3,1],[1,5,1],[4,2,1]]
Output: 7
Explanation: Because the path 1 → 3 → 1 → 1 → 1 minimizes the sum.
```

```
Input: grid = [[1,2,3],[4,5,6]]
Output: 12
```

### 题目解析

与前两题类似的状态转移矩阵，矩阵的当前值为左和上较小的一个加上当前值。

### 代码实现

###### c++

```c++
#include <vector>
#include <algorithm>
using namespace std;

class Solution {
public:
    int minPathSum(vector<vector<int>>& grid) {
        int m = grid.size();
        int n = grid[0].size();

        for (int j = 1; j < n; j++) {
            grid[0][j] += grid[0][j - 1];
        }
        for (int i = 1; i < m; i++) {
            grid[i][0] += grid[i - 1][0];
        }

        for (int i = 1; i < m; i++) {
            for (int j = 1; j < n; j++) {
                grid[i][j] += min(grid[i - 1][j], grid[i][j - 1]);
            }
        }

        return grid[m - 1][n - 1];
    }
};
```