# 1391. Check if There is a Valid Path in a Grid

**Tags:** DFS

### Description

You are given an m x n grid. Each cell of grid represents a street. The street of grid[i][j] can be:

- 1 which means a street connecting the left cell and the right cell.
- 2 which means a street connecting the upper cell and the lower cell.
- 3 which means a street connecting the left cell and the lower cell.
- 4 which means a street connecting the right cell and the lower cell.
- 5 which means a street connecting the left cell and the upper cell.
- 6 which means a street connecting the right cell and the upper cell.

![](./main.png)

You will initially start at the street of the upper-left cell (0, 0). A valid path in the grid is a path that starts from the upper left cell (0, 0) and ends at the bottom-right cell (m - 1, n - 1). The path should only follow the streets.

Notice that you are not allowed to change any street.

Return true if there is a valid path in the grid or false otherwise.

### Example

###### Example I

![](./e1.png)

> Input: grid = [[2,4,3],[6,5,2]]
> Output: true
> Explanation: As shown you can start at cell (0, 0) and visit all the cells of the grid to reach (m - 1, n - 1).

###### Example II

![](./e2.png)

> Input: grid = [[1,2,1],[1,2,1]]
> Output: false
> Explanation: As shown you the street at cell (0, 0) is not connected with any street of any other cell and you will get stuck at cell (0, 0)

###### Example III

> Input: grid = [[1,1,2]]
> Output: false
> Explanation: You will get stuck at cell (0, 1) and you cannot reach cell (0, 2).

### Solution

深度优先搜索。

```c++
class Solution {
public:
    bool hasValidPath(vector<vector<int>>& grid) {
        int m = grid.size(), n = grid[0].size();
        vector<vector<bool>> visited(m, vector<bool>(n, false));
        return dfs(grid, visited, 0, 0);
    }

private:
    bool dfs(vector<vector<int>>& grid, vector<vector<bool>>& visited, int i, int j) {
        int m = grid.size(), n = grid[0].size();
        if (i < 0 || i >= m || j < 0 || j >= n || visited[i][j]) return false;
        if (i == m - 1 && j == n - 1) return true;

        visited[i][j] = true;
        bool up = false, down = false, left = false, right = false;
        if (j + 1 < n && (grid[i][j] == 1 || grid[i][j] == 4 || grid[i][j] == 6) && (grid[i][j + 1] == 1 || grid[i][j + 1] == 3 || grid[i][j + 1] == 5)) {
            right = dfs(grid, visited, i, j + 1);
        }
        if (j - 1 > -1 &&  (grid[i][j] == 1 || grid[i][j] == 3 || grid[i][j] == 5) && (grid[i][j - 1] == 1 || grid[i][j - 1] == 4 || grid[i][j - 1] == 6)) {
            left = dfs(grid, visited, i, j - 1);
        }
        if (i + 1 < m && (grid[i][j] == 2 || grid[i][j] == 3 || grid[i][j] == 4) && (grid[i + 1][j] == 2 || grid[i + 1][j] == 5 || grid[i + 1][j] == 6)) {
            down = dfs(grid, visited, i + 1, j);
        }
        if (i - 1 > -1 && (grid[i][j] == 2 || grid[i][j] == 5 || grid[i][j] == 6) && (grid[i - 1][j] == 2 || grid[i - 1][j] == 3 || grid[i - 1][j] == 4)) {
            up = dfs(grid, visited, i - 1, j);
        }
        return up || down || left || right;
    }
};
```
