# 1020. Number of Enclaves

### Description

You are given an m x n binary matrix grid, where 0 represents a sea cell and 1 represents a land cell.

A move consists of walking from one land cell to another adjacent (4-directionally) land cell or walking off the boundary of the grid.

Return the number of land cells in grid for which we cannot walk off the boundary of the grid in any number of moves.

### Example

###### Example I

![](./enclaves1.jpg)

> Input: grid = [[0,0,0,0],[1,0,1,0],[0,1,1,0],[0,0,0,0]]
> Output: 3
> Explanation: There are three 1s that are enclosed by 0s, and one 1 that is not enclosed because its on the boundary.

###### Example II

![](./enclaves2.jpg)

> Input: grid = [[0,1,1,0],[0,0,1,0],[0,0,1,0],[0,0,0,0]]
> Output: 0
> Explanation: All 1s are either on the boundary or can reach the boundary.

### Solution

深度优先搜索。

```c++
class Solution {
public:
    int numEnclaves(vector<vector<int>>& grid) {
        int m = grid.size(), n = grid[0].size();
        vector<vector<bool>> visited(m, vector<bool>(n, false));
        int result = 0;
        
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                if (grid[i][j] == 1 && !visited[i][j]) {
                    int count = 0;
                    bool isEnclave = dfs(grid, visited, i, j, count);
                    if (isEnclave) {
                        result += count;
                    }
                }
            }
        }
        return result;
    }

private:
    bool dfs(vector<vector<int>>& grid, vector<vector<bool>>& visited, int i, int j, int& count) {
        int m = grid.size(), n = grid[0].size();
        
        if (i < 0 || i >= m || j < 0 || j >= n) return false;
        if (grid[i][j] == 0 || visited[i][j]) return true;
        
        visited[i][j] = true;
        count++;

        bool left = dfs(grid, visited, i, j - 1, count);
        bool right = dfs(grid, visited, i, j + 1, count);
        bool up = dfs(grid, visited, i - 1, j, count);
        bool down = dfs(grid, visited, i + 1, j, count);
        
        return left && right && up && down;
    }
};
```
