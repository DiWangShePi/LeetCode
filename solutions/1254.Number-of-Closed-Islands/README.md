# 1254. Number of Closed Islands

**Tags:** DFS

### Description

Given a 2D grid consists of 0s (land) and 1s (water).  An island is a maximal 4-directionally connected group of 0s and a closed island is an island totally (all left, top, right, bottom) surrounded by 1s.

Return the number of closed islands.

### Example

###### Example I

![](./sample_3_1610.png)

> Input: grid = [[1,1,1,1,1,1,1,0],[1,0,0,0,0,1,1,0],[1,0,1,0,1,1,1,0],[1,0,0,0,0,1,0,1],[1,1,1,1,1,1,1,0]]
> Output: 2
> Explanation: 
> Islands in gray are closed because they are completely surrounded by water (group of 1s).

###### Example II

![](./sample_4_1610.png)

> Input: grid = [[0,0,1,0,0],[0,1,0,1,0],[0,1,1,1,0]]
> Output: 1

###### Example III

> Input: grid = [[1,1,1,1,1,1,1],
>                [1,0,0,0,0,0,1],
>                [1,0,1,1,1,0,1],
>                [1,0,1,0,1,0,1],
>                [1,0,1,1,1,0,1],
>                [1,0,0,0,0,0,1],
>                [1,1,1,1,1,1,1]]
> Output: 2

### Solution

深度优先搜索，确定每一个岛是不是被环绕的。

```c++
class Solution {
public:
    int closedIsland(vector<vector<int>>& grid) {
        int an = 0;
        int m = grid.size(), n = grid[0].size();
        vector<vector<bool>> visited(m, vector<bool>(n, false));
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                if (grid[i][j] == 0 && !visited[i][j]) {
                    if (dfs(grid, visited, i , j)) an++;
                }
            }
        }
        return an;
    }

private:
    bool dfs(vector<vector<int>>& grid, vector<vector<bool>>& visited, int i, int j) {
        int m = grid.size(), n = grid[0].size();
        if (i < 0 || i >= m || j < 0 || j >= n) return false;
        if (visited[i][j]) return true;

        visited[i][j] = true;
        if (grid[i][j] == 1) return true;
        bool up = dfs(grid, visited, i - 1, j);
        bool down = dfs(grid, visited, i + 1, j);
        bool left = dfs(grid, visited, i, j - 1);
        bool right = dfs(grid, visited, i, j + 1);
        return up && down && left && right;
    }
};
```
