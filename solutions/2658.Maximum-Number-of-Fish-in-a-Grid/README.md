# 2658. Maximum Number of Fish in a Grid

**Tags:** DFS

### Description

You are given a 0-indexed 2D matrix grid of size m x n, where (r, c) represents:

- A land cell if grid[r][c] = 0, or
- A water cell containing grid[r][c] fish, if grid[r][c] > 0.
A fisher can start at any water cell (r, c) and can do the following operations any number of times:

- Catch all the fish at cell (r, c), or
- Move to any adjacent water cell.
Return the maximum number of fish the fisher can catch if he chooses his starting cell optimally, or 0 if no water cell exists.

An adjacent cell of the cell (r, c), is one of the cells (r, c + 1), (r, c - 1), (r + 1, c) or (r - 1, c) if it exists.

### Example

###### Example I

![](./example.png)

> Input: grid = [[0,2,1,0],[4,0,0,3],[1,0,0,4],[0,3,2,0]]
> Output: 7
> Explanation: The fisher can start at cell (1,3) and collect 3 fish, then move to cell (2,3) and collect 4 fish.

###### Example II

![](./example2.png)

> Input: grid = [[1,0,0,0],[0,0,0,0],[0,0,0,0],[0,0,0,1]]
> Output: 1
> Explanation: The fisher can start at cells (0,0) or (3,3) and collect a single fish. 

### Solution

遍历加深度优先搜索

```c++
class Solution {
public:
    int findMaxFish(vector<vector<int>>& grid) {
        int m = grid.size(), n = grid[0].size(), an = 0;
        vector<vector<bool>> visited(m, vector<bool>(n, false));
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                if (grid[i][j] != 0 && !visited[i][j]) {
                    int t = dfs(grid, visited, i, j);
                    an = max(t, an);
                }
            }
        }
        return an;
    }

private:
    int dfs(vector<vector<int>>& grid, vector<vector<bool>>& visited, int i, int j) {
        int m = grid.size(), n = grid[0].size();
        if (i < 0 || i >= m || j < 0 || j >= n || visited[i][j] || grid[i][j] == 0)
            return 0;

        int an = grid[i][j];
        visited[i][j] = true;
        an += dfs(grid, visited, i + 1, j);
        an += dfs(grid, visited, i - 1, j);
        an += dfs(grid, visited, i, j + 1);
        an += dfs(grid, visited, i, j - 1);
        return an;
    }
};
```
