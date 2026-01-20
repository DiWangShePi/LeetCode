# 1765. Map of Highest Peak

**Tags:** BFS

### Description

You are given an integer matrix isWater of size m x n that represents a map of land and water cells.

- If isWater[i][j] == 0, cell (i, j) is a land cell.
- If isWater[i][j] == 1, cell (i, j) is a water cell.
You must assign each cell a height in a way that follows these rules:

- The height of each cell must be non-negative.
- If the cell is a water cell, its height must be 0.
- Any two adjacent cells must have an absolute height difference of at most 1. A cell is adjacent to another cell if the former is directly north, east, south, or west of the latter (i.e., their sides are touching).
Find an assignment of heights such that the maximum height in the matrix is maximized.

Return an integer matrix height of size m x n where height[i][j] is cell (i, j)'s height. If there are multiple solutions, return any of them.

### Example

###### Example I

![](./screenshot-2021-01-11-at-82045-am.png)

> Input: isWater = [[0,1],[0,0]]
> Output: [[1,0],[2,1]]
> Explanation: The image shows the assigned heights of each cell.
> The blue cell is the water cell, and the green cells are the land cells.

###### Example II

![](./screenshot-2021-01-11-at-82050-am.png)

> Input: isWater = [[0,0,1],[1,0,0],[0,0,0]]
> Output: [[1,1,0],[0,1,1],[1,2,2]]
> Explanation: A height of 2 is the maximum possible height of any assignment.
> Any height assignment that has a maximum height of 2 while still meeting the rules will also be accepted.

### Solution

广度优先搜索。

```c++
class Solution {
public:
    vector<vector<int>> highestPeak(vector<vector<int>>& isWater) {
        int m = isWater.size(), n = isWater[0].size();
        vector<vector<int>> queue;
        vector<vector<int>> ans(m, vector<int>(n, 0));
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                if (isWater[i][j] == 1) queue.push_back({i, j});
            }
        }

        int l = 0, r = queue.size(), h = 1;
        vector<vector<int>> dir{{0, -1}, {-1, 0}, {0, 1}, {1, 0}};
        while (l < r) {
            for (int i = l; i < r; i++) {
                int cx = queue[i][0], cy = queue[i][1];
                for (vector<int>& d : dir) {
                    int nx = cx + d[0], ny = cy + d[1];
                    if (nx < 0 || nx >= m || ny < 0 || ny >= n || isWater[nx][ny] == 1) continue;

                    isWater[nx][ny] = 1;
                    queue.push_back({nx, ny});
                    ans[nx][ny] = h;
                }
            }

            h++;
            l = r;
            r = queue.size();
        }
        return ans;
    }
};
```
