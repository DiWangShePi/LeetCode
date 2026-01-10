# 2684. Maximum Number of Moves in a Grid

**Tags:** DFS

### Desription

You are given a 0-indexed m x n matrix grid consisting of positive integers.

You can start at any cell in the first column of the matrix, and traverse the grid in the following way:

- From a cell (row, col), you can move to any of the cells: (row - 1, col + 1), (row, col + 1) and (row + 1, col + 1) such that the value of the cell you move to, should be strictly bigger than the value of the current cell.
Return the maximum number of moves that you can perform.

### Example

###### Example I

![](./yetgriddrawio-10.png)

> Input: grid = [[2,4,3,5],[5,4,9,3],[3,4,2,11],[10,9,13,15]]
> Output: 3
> Explanation: We can start at the cell (0, 0) and make the following moves:
> - (0, 0) -> (0, 1).
> - (0, 1) -> (1, 2).
> - (1, 2) -> (2, 3).
> It can be shown that it is the maximum number of moves that can be made.

###### Example II

![](./yetgrid4drawio.png)

> Input: grid = [[3,2,4],[2,1,9],[1,1,7]]
> Output: 0
> Explanation: Starting from any cell in the first column we cannot perform any moves.

### Solution

深度优先遍历加记忆化搜索。

```c++
class Solution {
public:
    int maxMoves(vector<vector<int>>& grid) {
        int m = grid.size(), n = grid[0].size();
        vector<vector<int>> mem(m, vector<int>(n, 0));

        int an = 0;
        for (int i = 0; i < grid.size(); i++)
            an = max(an, dfs(grid, mem, i, 0) - 1);
        return an;
    }

private:
    int dfs(vector<vector<int>>& grid, vector<vector<int>>& mem, int i, int j) {
        if (mem[i][j] != 0) return mem[i][j];

        int m = grid.size(), n = grid[0].size();
        int an = 1, t = 0;
        if (j + 1 < n  && grid[i][j] < grid[i][j + 1]) t = max(t, dfs(grid, mem, i, j + 1));
        if (i - 1 > -1 && j + 1 < n && grid[i][j] < grid[i - 1][j + 1]) t = max(t, dfs(grid, mem, i - 1, j + 1));
        if (i + 1 < m  && j + 1 < n && grid[i][j] < grid[i + 1][j + 1]) t = max(t, dfs(grid, mem, i + 1, j + 1));
        mem[i][j] = an + t;
        return an + t;
    }
};
```
