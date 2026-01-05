# 3619. Count Islands With Total Value Divisible by K

**Tags:** DFS

### Description

You are given an m x n matrix grid and a positive integer k. An island is a group of positive integers (representing land) that are 4-directionally connected (horizontally or vertically).

The total value of an island is the sum of the values of all cells in the island.

Return the number of islands with a total value divisible by k.

### Example

###### Example I

![](./example1griddrawio-1.png)

> Input: grid = [[0,2,1,0,0],[0,5,0,0,5],[0,0,1,0,0],[0,1,4,7,0],[0,2,0,0,8]], k = 5
> Output: 2
> Explanation:
> The grid contains four islands. The islands highlighted in blue have a total value that is divisible by 5, while the islands highlighted in red do not.

###### Example II

![](./example2griddrawio.png)

> Input: grid = [[3,0,3,0], [0,3,0,3], [3,0,3,0]], k = 3
> Output: 6
> Explanation:
> The grid contains six islands, each with a total value that is divisible by 3.

### Solution

深度优先搜索。

```c++
class Solution {
public:
    int countIslands(vector<vector<int>>& grid, int k) {
        int m = grid.size(), n = grid[0].size();
        int an = 0;
        vector<vector<bool>> visited(m, vector<bool>(n, false));
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                if (grid[i][j] != 0 && !visited[i][j]) {
                    long t = dfs(grid, visited, i, j);
                    an = t % k == 0 ? an + 1 : an;
                }
            }
        }
        return an;
    }

private:
    long dfs(vector<vector<int>>& grid, vector<vector<bool>>& visited, int i, int j) {
        int m = grid.size(), n = grid[0].size();
        if (i < 0 || i >= m || j < 0 || j >= n || visited[i][j] || grid[i][j] == 0) return 0;

        long an = grid[i][j];
        visited[i][j] = true;
        an += dfs(grid, visited, i + 1, j);
        an += dfs(grid, visited, i - 1, j);
        an += dfs(grid, visited, i, j + 1);
        an += dfs(grid, visited, i, j - 1);
        return an;
    }
};
```
