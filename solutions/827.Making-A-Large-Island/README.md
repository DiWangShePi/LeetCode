# 827. Making A Large Island

**Tags:** DFS

### Description

You are given an n x n binary matrix grid. You are allowed to change at most one 0 to be 1.

Return the size of the largest island in grid after applying this operation.

An island is a 4-directionally connected group of 1s.

### Example

###### Example I

> Input: grid = [[1,0],[0,1]]
> Output: 3
> Explanation: Change one 0 to 1 and connect two 1s, then we get an island with area = 3.

###### Example II

> Input: grid = [[1,1],[1,0]]
> Output: 4
> Explanation: Change the 0 to 1 and make the island bigger, only one island with area = 4.

###### Example III

> Input: grid = [[1,1],[1,1]]
> Output: 4
> Explanation: Can't change any 0 to 1, only one island with area = 4.

### Solution

每找到一个岛记录岛的面积和每个岛的位置，随后遍历所有 0 的位置，考虑周围的四个位置是否属于同一个岛屿，不属于时累加他们的面积。

```c++
class Solution {
public:
    int largestIsland(vector<vector<int>>& grid) {
        int m = grid.size(), n = grid[0].size(), t = 0, an = 0;
        vector<vector<bool>> visited(m, vector<bool>(n, false));
        vector<vector<int>> island(m, vector<int>(n, 0));
        vector<vector<int>> identity(m, vector<int>(n, 0));
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                if (grid[i][j] == 1 && !visited[i][j]) {
                    vector<vector<int>> queue;
                    int m = dfs(grid, visited, i, j, queue);
                    an = max(an, m);

                    t++;
                    for (vector<int>& q : queue) {
                        int idx = q[0], idy = q[1];
                        island[idx][idy] = m;
                        identity[idx][idy] = t;
                    }
                }
            }
        }

        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                if (grid[i][j] == 0) {
                    int c = cal(island, identity, i, j);
                    an = max(an, c);
                }
            }
        }
        return an;
    }

private:
    int dfs(vector<vector<int>>& grid, vector<vector<bool>>& visited, int i, int j, vector<vector<int>>& queue) {
        int m = grid.size(), n = grid[0].size();
        if (i < 0 || i >= m || j < 0 || j >= n || visited[i][j] || grid[i][j] == 0) return 0;

        int an = 1;
        visited[i][j] = true;
        queue.push_back({i, j});
        an += dfs(grid, visited, i + 1, j, queue);
        an += dfs(grid, visited, i - 1, j, queue);
        an += dfs(grid, visited, i, j + 1, queue);
        an += dfs(grid, visited, i, j - 1, queue);
        return an;
    }

    int cal(vector<vector<int>>& island, vector<vector<int>>& identity, int i, int j) {
        int m = island.size(), n = island[0].size(), an = 0;
        unordered_map<int, int> dict;
        if (i + 1 < m && dict.count(identity[i + 1][j]) == 0) {
            an += island[i + 1][j];
            dict[identity[i + 1][j]] = 0;
        }
        if (i - 1 > -1 && dict.count(identity[i - 1][j]) == 0) {
            an += island[i - 1][j];
            dict[identity[i - 1][j]] = 0;
        }
        if (j + 1 < n && dict.count(identity[i][j + 1]) == 0) {
            an += island[i][j + 1];
            dict[identity[i][j + 1]] = 0;
        }
        if (j - 1 > -1 && dict.count(identity[i][j - 1]) == 0) {
            an += island[i][j - 1];
            dict[identity[i][j - 1]] = 0;
        }
        return an + 1;
    }
};
```