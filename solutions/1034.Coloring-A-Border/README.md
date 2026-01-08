# 1034. Coloring A Border

**Tags:** DFS

### Description

You are given an m x n integer matrix grid, and three integers row, col, and color. Each value in the grid represents the color of the grid square at that location.

Two squares are called adjacent if they are next to each other in any of the 4 directions.

Two squares belong to the same connected component if they have the same color and they are adjacent.

The border of a connected component is all the squares in the connected component that are either adjacent to (at least) a square not in the component, or on the boundary of the grid (the first or last row or column).

You should color the border of the connected component that contains the square grid[row][col] with color.

Return the final grid.

### Example

###### Example I

> Input: grid = [[1,1],[1,2]], row = 0, col = 0, color = 3
> Output: [[3,3],[3,2]]

###### Example II

> Input: grid = [[1,2,2],[2,3,2]], row = 0, col = 1, color = 3
> Output: [[1,3,3],[2,3,3]]

###### Example III

> Input: grid = [[1,1,1],[1,1,1],[1,1,1]], row = 1, col = 1, color = 2
> Output: [[2,2,2],[2,1,2],[2,2,2]]

### Solution

从给定的地方开始，深度优先搜索。

```c++
class Solution {
    int m, n;
public:
    vector<vector<int>> colorBorder(vector<vector<int>>& grid, int row, int col, int color) {
        m = grid.size();
        n = grid[0].size();
        vector<vector<bool>> visited(m, vector<bool>(n, false));
        vector<vector<int>> ans;
        dfs(grid, visited, row, col, grid[row][col], ans);
        for (vector<int>& an : ans) {
            int i = an[0], j = an[1];
            grid[i][j] = color;
        }
        return grid;
    }

private:
    void dfs(vector<vector<int>>& grid, vector<vector<bool>>& visited, int i, int j, int origin, vector<vector<int>>& an) {
        if (i < 0 || i >= m || j < 0 || j >= n || visited[i][j]) return;

        visited[i][j] = true;
        bool is = grid[i][j] == origin;
        if (isEdge(grid, i, j, origin) && is) an.push_back({i, j});
        if (!is) return;
        dfs(grid, visited, i + 1, j, origin, an);
        dfs(grid, visited, i - 1, j, origin, an);
        dfs(grid, visited, i, j + 1, origin, an);
        dfs(grid, visited, i, j - 1, origin, an);
    }

    bool isEdge(vector<vector<int>>& grid, int i, int j, int origin) {
        if (i + 1 >= m || grid[i + 1][j] != origin) return true;
        if (i - 1 < 0 || grid[i - 1][j] != origin) return true;
        if (j + 1 >= n || grid[i][j + 1] != origin) return true;
        if (j - 1 < 0 || grid[i][j - 1] != origin) return true;
        return false;
    }
};
```
