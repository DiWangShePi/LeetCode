# 1091. Shortest Path in Binary Matrix

**Tags:** BFS

### Description

Given an n x n binary matrix grid, return the length of the shortest clear path in the matrix. If there is no clear path, return -1.

A clear path in a binary matrix is a path from the top-left cell (i.e., (0, 0)) to the bottom-right cell (i.e., (n - 1, n - 1)) such that:

- All the visited cells of the path are 0.
- All the adjacent cells of the path are 8-directionally connected (i.e., they are different and they share an edge or a corner).
The length of a clear path is the number of visited cells of this path.

### Example

###### Example I

![](./example1_1.png)

> Input: grid = [[0,1],[1,0]]
> Output: 2

###### Example II

![](./example2_1.png)

> Input: grid = [[0,0,0],[1,1,0],[1,1,0]]
> Output: 4

###### Example III

> Input: grid = [[1,0,0],[1,1,0],[1,1,0]]
> Output: -1

### Solution

广度优先搜索。

```c++
class Solution {
public:
    int shortestPathBinaryMatrix(vector<vector<int>>& grid) {
         int m = grid.size(), n = grid[0].size();
        if (grid[0][0] == 1 || grid[m - 1][n - 1]) return -1;

        vector<vector<int>> queue;
        vector<vector<bool>> visited(m, vector<bool>(n, false));
        vector<int> dx{-1,-1,-1,0,0,1,1,1}, dy{-1,0,1,-1,1,-1,0,1};

        int l = 0, r = 1, w = 1;
        queue.push_back({0, 0});
        visited[0][0] = true;
        while (l != r) {
            w += 1;
            for (int i = l; i < r; i++) {
                int cx = queue[i][0], cy = queue[i][1];
                for (int j = 0; j < 8; j++) {
                    int nx = cx + dx[j], ny = cy + dy[j];
                    if (nx < 0 || nx >= m || ny < 0 || ny >= n || grid[nx][ny] == 1 || visited[nx][ny]) continue;
                    if (nx == m - 1 && ny == n - 1) return w;

                    visited[nx][ny] = true;
                    queue.push_back({nx, ny});
                }
            }

            l = r;
            r = queue.size();
        }
        return visited[m - 1][n - 1] ? w - 1 : -1;
    }
};
```