# 1905. Count Sub Islands

**Tags:** DFS

### Description

You are given two m x n binary matrices grid1 and grid2 containing only 0's (representing water) and 1's (representing land). An island is a group of 1's connected 4-directionally (horizontal or vertical). Any cells outside of the grid are considered water cells.

An island in grid2 is considered a sub-island if there is an island in grid1 that contains all the cells that make up this island in grid2.

Return the number of islands in grid2 that are considered sub-islands.

### Example

###### Example I

![](./test1.png)

> Input: grid1 = [[1,1,1,0,0],[0,1,1,1,1],[0,0,0,0,0],[1,0,0,0,0],[1,1,0,1,1]], grid2 = [[1,1,1,0,0],[0,0,1,1,1],[0,1,0,0,0],[1,0,1,1,0],[0,1,0,1,0]]
> Output: 3
> Explanation: In the picture above, the grid on the left is grid1 and the grid on the right is grid2.
> The 1s colored red in grid2 are those considered to be part of a sub-island. There are three sub-islands.

###### Example II

![](./testcasex2.png)

> Input: grid1 = [[1,0,1,0,1],[1,1,1,1,1],[0,0,0,0,0],[1,1,1,1,1],[1,0,1,0,1]], grid2 = [[0,0,0,0,0],[1,1,1,1,1],[0,1,0,1,0],[0,1,0,1,0],[1,0,0,0,1]]
> Output: 2 
> Explanation: In the picture above, the grid on the left is grid1 and the grid on the right is grid2.
> The 1s colored red in grid2 are those considered to be part of a sub-island. There are two sub-islands.

### Solution

深度优先搜索。

```c++
class Solution {
public:
    int countSubIslands(vector<vector<int>>& grid1, vector<vector<int>>& grid2) {
        int m = grid2.size(), n = grid2[0].size();
        vector<vector<bool>> visited(m, vector<bool>(n, false));

        int an = 0;
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                if (grid2[i][j] == 1 && !visited[i][j]) {
                    if (dfs(grid1, grid2, visited, i, j)) an++;
                }
            }
        }
        return an;
    }

private:
    bool dfs(vector<vector<int>>& grid1, vector<vector<int>>& grid2, vector<vector<bool>>& visited, int i, int j) {
        int m = grid2.size(), n = grid2[0].size();
        if (i < 0 || i >= m || j < 0 || j >= n || visited[i][j] || grid2[i][j] == 0) return true;
        if (grid1[i][j] != 1) return false;

        visited[i][j] = true;
        bool up = dfs(grid1, grid2, visited, i - 1, j);
        bool down = dfs(grid1, grid2, visited, i + 1, j);
        bool right = dfs(grid1, grid2, visited, i, j + 1);
        bool left = dfs(grid1, grid2, visited, i, j - 1);
        return up && down && right && left;
    }
};
```
