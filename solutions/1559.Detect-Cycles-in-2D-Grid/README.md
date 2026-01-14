# 1559. Detect Cycles in 2D Grid

**Tags:** DFS

### Description

Given a 2D array of characters grid of size m x n, you need to find if there exists any cycle consisting of the same value in grid.

A cycle is a path of length 4 or more in the grid that starts and ends at the same cell. From a given cell, you can move to one of the cells adjacent to it - in one of the four directions (up, down, left, or right), if it has the same value of the current cell.

Also, you cannot move to the cell that you visited in your last move. For example, the cycle (1, 1) -> (1, 2) -> (1, 1) is invalid because from (1, 2) we visited (1, 1) which was the last visited cell.

Return true if any cycle of the same value exists in grid, otherwise, return false.

### Example

###### Example I

![](./1.png)

> Input: grid = [["a","a","a","a"],["a","b","b","a"],["a","b","b","a"],["a","a","a","a"]]
> Output: true
> Explanation: There are two valid cycles shown in different colors in the image below:
> ![](./11.png)

###### Example II

![](./22.png)

> Input: grid = [["c","c","c","a"],["c","d","c","c"],["c","c","e","c"],["f","c","c","c"]]
> Output: true
> Explanation: There is only one valid cycle highlighted in the image below:
> ![](./2.png)

###### Example III

![](./3.png)

> Input: grid = [["a","b","b"],["b","z","b"],["b","b","a"]]
> Output: false

### Solution

深度优先搜索，两个变量记录自己从哪里来，确保不走回头路。visited 数组确认是否有环。

```c++
class Solution {
public:
    bool containsCycle(vector<vector<char>>& grid) {
        int m = grid.size(), n = grid[0].size();
        vector<vector<bool>> visited(m, vector<bool>(n, false));
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                if (!visited[i][j]) {
                    if (dfs(grid, visited, i, j, grid[i][j], -1, -1)) return true;
                }
                
            }
        }
        return false;
    }

private:
    bool dfs(vector<vector<char>>& grid, vector<vector<bool>>& visited, int i, int j, char c, int li, int lj) {
        int m = grid.size(), n = grid[0].size();
        if (i < 0 || i >= m || j < 0 || j >= n || grid[i][j] != c) return false;
        if (visited[i][j]) return true;

        visited[i][j] = true;
        bool up = false, down = false, left = false, right = false;
        if (j + 1 != lj) right = dfs(grid, visited, i, j + 1, c, i, j);
        if (j - 1 != lj) left = dfs(grid, visited, i, j - 1, c, i, j);
        if (i + 1 != li) down = dfs(grid, visited, i + 1, j, c, i, j);
        if (i - 1 != li) up = dfs(grid, visited, i - 1, j, c, i, j);
        return up || down || left || right;
    }
};
```
