# 542. 01 Matrix

**Tags:** BFS

### Description

Given an m x n binary matrix mat, return the distance of the nearest 0 for each cell.

The distance between two cells sharing a common edge is 1.

### Example

###### Example I

![](./01-1-grid.jpg)

> Input: mat = [[0,0,0],[0,1,0],[0,0,0]]
> Output: [[0,0,0],[0,1,0],[0,0,0]]

###### Example II

> Input: mat = [[0,0,0],[0,1,0],[1,1,1]]
> Output: [[0,0,0],[0,1,0],[1,2,1]]

### Solution

广度优先搜索

```c++
class Solution {
public:
    vector<vector<int>> updateMatrix(vector<vector<int>>& mat) {
        int m = mat.size(), n = mat[0].size();
        vector<vector<int>> dist(m, vector<int>(n, -1));
        queue<pair<int, int>> q;
        
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                if (mat[i][j] == 0) {
                    dist[i][j] = 0;
                    q.push({i, j});
                }
            }
        }
        
        vector<pair<int, int>> directions = {{1, 0}, {-1, 0}, {0, 1}, {0, -1}};
        
        while (!q.empty()) {
            auto [i, j] = q.front();
            q.pop();
            
            for (auto [di, dj] : directions) {
                int ni = i + di, nj = j + dj;
                if (ni >= 0 && ni < m && nj >= 0 && nj < n && dist[ni][nj] == -1) {
                    dist[ni][nj] = dist[i][j] + 1;
                    q.push({ni, nj});
                }
            }
        }
        
        return dist;
    }
};
```
