# 200. Number of Islands

### Description

Given an m x n 2D binary grid grid which represents a map of '1's (land) and '0's (water), return the number of islands.

An island is surrounded by water and is formed by connecting adjacent lands horizontally or vertically. You may assume all four edges of the grid are all surrounded by water.

### Solution

遍历，每遇到一个“1”用递归检查其临近的节点是否同样为“1”，将遇到的所有“1“标记为遇到过了。此后每一个未遇到且为”1“的至少为一个新的岛屿。

### Implementation

###### c++

```c++
class Solution {
public:
    int numIslands(vector<vector<char>>& grid) {
        int m = grid.size();
        int n = grid[0].size();
        vector<vector<bool>> visited(m, vector<bool>(n, false));
        int result = 0;

        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                if (!visited[i][j]) {
                    if (grid[i][j] == '1') {
                        result++;
                        dfs(grid, i, j, m, n, visited);
                    }
                }
            }
        }
        return result;
    }

    void dfs(vector<vector<char>>& grid, int i, int j, int m, int n, vector<vector<bool>>& visited) {
        if (grid[i][j] == '0' || visited[i][j]) return;
        visited[i][j] = true;

        if (i + 1 < m)  dfs(grid, i+1, j, m, n, visited);
        if (i - 1 > -1) dfs(grid, i-1, j, m, n, visited);
        if (j + 1 < n)  dfs(grid, i, j+1, m, n, visited);
        if (j - 1 > -1) dfs(grid, i, j-1, m, n, visited);
    }
};
```
