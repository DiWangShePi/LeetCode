# 695. Max Area of Island

**Tags:** DFS

### Description

You are given an m x n binary matrix grid. An island is a group of 1's (representing land) connected 4-directionally (horizontal or vertical.) You may assume all four edges of the grid are surrounded by water.

The area of an island is the number of cells with a value 1 in the island.

Return the maximum area of an island in grid. If there is no island, return 0.

### Example

###### Example I

![](./maxarea1-grid.jpg)

> Input: grid = [[0,0,1,0,0,0,0,1,0,0,0,0,0],[0,0,0,0,0,0,0,1,1,1,0,0,0],[0,1,1,0,1,0,0,0,0,0,0,0,0],[0,1,0,0,1,1,0,0,1,0,1,0,0],[0,1,0,0,1,1,0,0,1,1,1,0,0],[0,0,0,0,0,0,0,0,0,0,1,0,0],[0,0,0,0,0,0,0,1,1,1,0,0,0],[0,0,0,0,0,0,0,1,1,0,0,0,0]]
> Output: 6
> Explanation: The answer is not 11, because the island must be connected 4-directionally.

###### Example II

> Input: grid = [[0,0,0,0,0,0,0,0]]
> Output: 0

### Solution

深度优先遍历，每次计算整个岛屿的面积，记录最大的。

```c++
class Solution {
public:
    int maxAreaOfIsland(vector<vector<int>>& grid) {
        int m = grid.size(), n = grid[0].size();
        int an = 0;
        vector<vector<bool>> visited(m, vector<bool>(n, false));
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                if (grid[i][j] == 1 && !visited[i][j]) {
                    int t = dfs(grid, visited, i, j);
                    an = max(an, t);
                }
            }
        }
        return an;
    }

private:
    int dfs(vector<vector<int>>& grid, vector<vector<bool>>& visited, int i, int j) {
        int m = grid.size(), n = grid[0].size();
        if (i < 0 || i >= m || j < 0 || j >= n || grid[i][j] == 0 || visited[i][j]) return 0;

        int an = 1;
        visited[i][j] = true;
        an += dfs(grid, visited, i + 1, j);
        an += dfs(grid, visited, i - 1, j);
        an += dfs(grid, visited, i, j + 1);
        an += dfs(grid, visited, i, j - 1);
        return an;
    }
};
```
