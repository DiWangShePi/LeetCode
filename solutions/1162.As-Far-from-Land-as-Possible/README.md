# 1162. As Far from Land as Possible

**Tags:** BFS

### Descritpion

Given an n x n grid containing only values 0 and 1, where 0 represents water and 1 represents land, find a water cell such that its distance to the nearest land cell is maximized, and return the distance. If no land or water exists in the grid, return -1.

The distance used in this problem is the Manhattan distance: the distance between two cells (x0, y0) and (x1, y1) is |x0 - x1| + |y0 - y1|.

### Example

###### Example I

![](./1336_ex1.jpg)

> Input: grid = [[1,0,1],[0,0,0],[1,0,1]]
> Output: 2
> Explanation: The cell (1, 1) is as far as possible from all the land with distance 2.

###### Example II

![](./1336_ex2.jpg)

> Input: grid = [[1,0,0],[0,0,0],[0,0,0]]
> Output: 4
> Explanation: The cell (2, 2) is as far as possible from all the land with distance 4.

### Solution

广度优先搜索。

```c++
class Solution {
public:
    int maxDistance(vector<vector<int>>& grid) {
        int m = grid.size(), n = grid[0].size();
        vector<vector<int>> queue;
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                if (grid[i][j] == 1) queue.push_back({i, j, 0});
            }
        }

        if (queue.size() == 0 || queue.size() == m * n) return -1;

        int l = 0, r = queue.size(), w = 0;
        vector<int> dx{-1,0,0,1}, dy{0,-1,1,-0};
        while (l != r) {
            for (int i = l; i < r; i++) {
                int cx = queue[i][0], cy = queue[i][1];
                for (int j = 0; j < 4; j++) {
                    int nx = cx + dx[j], ny = cy + dy[j];
                    if (nx < 0 || nx >= m || ny < 0 || ny >= n || grid[nx][ny] != 0) continue;

                    queue.push_back({nx, ny});
                    grid[nx][ny] = 1;
                }
            }

            w++;
            l = r;
            r = queue.size();
        }
        return w - 1;
    }
};
```
