# 994. Rotting Oranges

**Tags:** BFS

### Description

You are given an m x n grid where each cell can have one of three values:

- 0 representing an empty cell,
- 1 representing a fresh orange, or
- 2 representing a rotten orange.
Every minute, any fresh orange that is 4-directionally adjacent to a rotten orange becomes rotten.

Return the minimum number of minutes that must elapse until no cell has a fresh orange. If this is impossible, return -1.

### Example

###### Example I

![](./oranges.png)

> Input: grid = [[2,1,1],[1,1,0],[0,1,1]]
> Output: 4

###### Example II

> Input: grid = [[2,1,1],[0,1,1],[1,0,1]]
> Output: -1
> Explanation: The orange in the bottom left corner (row 2, column 0) is never rotten, because rotting only happens 4-directionally.

###### Example III

> Input: grid = [[0,2]]
> Output: 0
> Explanation: Since there are already no fresh oranges at minute 0, the answer is just 0.

### Solution

广度优先搜索，最后扫一遍有没有没腐烂的橘子。

```c++
class Solution {
public:
    int orangesRotting(vector<vector<int>>& grid) {
        int m = grid.size(), n = grid[0].size();
        vector<vector<int>> queue;
        int numO = 0;
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                if (grid[i][j] == 2) queue.push_back({i, j});
                if (grid[i][j] == 1) numO++;
            }
        }

        int time = 0;
        int l = 0, r = queue.size();
        vector<vector<int>> dir{{0, 1}, {1, 0}, {0, -1}, {-1, 0}};
        while (l < r) {
            for (int i = l; i < r; i++) {
                int cx = queue[i][0], cy = queue[i][1];
                for (vector<int>& d : dir) {
                    int nx = cx + d[0], ny = cy + d[1];
                    if (nx < 0 || nx >= m || ny < 0 || ny >= n || grid[nx][ny] != 1) continue;

                    grid[nx][ny] = 2;
                    numO--;
                    queue.push_back({nx, ny});
                }
            }

            
            l = r;
            r = queue.size();
            if (l != r) time++;
        }
        return numO == 0 ? time : -1;
    }
};
```
