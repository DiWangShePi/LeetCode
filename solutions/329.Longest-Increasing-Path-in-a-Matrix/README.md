# 329. Longest Increasing Path in a Matrix

### Description

Given an m x n integers matrix, return the length of the longest increasing path in matrix.

From each cell, you can either move in four directions: left, right, up, or down. You may not move diagonally or move outside the boundary (i.e., wrap-around is not allowed).

### Example 

###### Example I

![](./grid1.jpg)

```
Input: matrix = [[9,9,4],[6,6,8],[2,1,1]]
Output: 4
Explanation: The longest increasing path is [1, 2, 6, 9].
```

###### Example II

![](./tmp-grid.jpg)

```
Input: matrix = [[3,4,5],[3,2,6],[2,2,1]]
Output: 4
Explanation: The longest increasing path is [3, 4, 5, 6]. Moving diagonally is not allowed.
```

###### Example III

```
Input: matrix = [[1]]
Output: 1
```

### Solution

先给个暴力的方法：给定矩阵，对每个位置，尝试其有可能产生的最长路径，记录其长度

```c++
class Solution {
public:
    int longestIncreasingPath(vector<vector<int>>& matrix) {
        int m = matrix.size();
        int n = matrix[0].size();
        int result = 0;
        vector<vector<int>> visited(m, vector<int>(n, false));

        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                int count = 1;

                visited[i][j] = true;
                longestPath(matrix, visited, i, j, 1, count);
                visited[i][j] = false;

                if (count > result) result = count;
            }
        }
        return result;
    }

private:
    vector<vector<int>> direction{
        {-1, 0}, {0, -1}, {0, 1}, {1, 0}
    };

    bool legal(int m, int n, int i, int j, int dir_i, int dir_j) {
        if (i + dir_i < 0 || i + dir_i >= m) return false;
        if (j + dir_j < 0 || j + dir_j >= n) return false;
        return true;
    }

    void longestPath(vector<vector<int>>& matrix, vector<vector<int>>& visited, int i, int j, int path, int& count) {
        if (path > count) count = path;

        int m = matrix.size();
        int n = matrix[0].size();
        for (vector<int>& dir : direction) {
            int dir_i = dir[0], dir_j = dir[1];

            if (!legal(m, n, i, j, dir_i, dir_j)) continue;

            if (!visited[i + dir_i][j + dir_j] && matrix[i][j] < matrix[i + dir_i][j + dir_j]) {

                visited[i + dir_i][j + dir_j] = true;
                longestPath(matrix, visited, i + dir_i, j + dir_j, path + 1, count);
                visited[i + dir_i][j + dir_j] = false;
            }
        }
    }
};
```

当然，这样的解法会遇到TLE(Time Limit Exceeded)

所以我们需要尝试更优雅一点的解法。比如动态规划，将已经搜索过的格子的最长路径的长度保留下来，就不必在再次遇到的时候重复计算

```c++
class Solution {
public:
    int longestIncreasingPath(vector<vector<int>>& matrix) {
        int m = matrix.size();
        int n = matrix[0].size();
        vector<vector<int>> dp(m, vector<int>(n, 0));
        int result = 0;

        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                result = max(result, dfs(matrix, dp, i, j));
            }
        }

        return result;
    }

private:
    vector<vector<int>> directions = {{-1,0}, {1,0}, {0,-1}, {0,1}};

    int dfs(vector<vector<int>>& matrix, vector<vector<int>>& dp, int i, int j) {
        if (dp[i][j] != 0) return dp[i][j];

        int m = matrix.size(), n = matrix[0].size();
        int maxLen = 1;

        for (auto& dir : directions) {
            int ni = i + dir[0], nj = j + dir[1];
            if (ni >= 0 && ni < m && nj >= 0 && nj < n && matrix[ni][nj] > matrix[i][j]) {
                maxLen = max(maxLen, 1 + dfs(matrix, dp, ni, nj));
            }
        }

        dp[i][j] = maxLen;
        return maxLen;
    }
};
```

我们还可以将矩阵视为一个有向无环图（DAG），每个格子向比自己大的相邻格子连边，计算每个格子的出度。将所有出度为 0（即无法再递增）的格子作为起点入队，逐层进行广度优先遍历，每遍历一层代表路径增长一步，同时减少其前驱格子的出度，当某个前驱出度减为 0 时入队。最终遍历的层数即为矩阵中最长递增路径的长度。

```c++
class Solution {
public:
    int longestIncreasingPath(vector<vector<int>>& matrix) {
        int m = matrix.size(), n = matrix[0].size();
        vector<vector<int>> outDegree(m, vector<int>(n, 0));
        vector<vector<int>> dirs = {{-1,0},{1,0},{0,-1},{0,1}};

        // 计算每个点的出度
        for (int i = 0; i < m; ++i) {
            for (int j = 0; j < n; ++j) {
                for (auto& d : dirs) {
                    int ni = i + d[0], nj = j + d[1];
                    if (ni >= 0 && ni < m && nj >= 0 && nj < n && matrix[ni][nj] > matrix[i][j]) {
                        outDegree[i][j]++;
                    }
                }
            }
        }

        // 初始化队列：将所有出度为0的点放入（这些是终点）
        queue<pair<int, int>> q;
        for (int i = 0; i < m; ++i)
            for (int j = 0; j < n; ++j)
                if (outDegree[i][j] == 0)
                    q.emplace(i, j);

        int pathLength = 0;
        while (!q.empty()) {
            int sz = q.size();
            pathLength++; // 每一层代表一条路径长度
            for (int k = 0; k < sz; ++k) {
                auto [i, j] = q.front(); q.pop();
                for (auto& d : dirs) {
                    int ni = i + d[0], nj = j + d[1];
                    if (ni >= 0 && ni < m && nj >= 0 && nj < n && matrix[ni][nj] < matrix[i][j]) {
                        // 反向地看：从更大值走向更小值，减少其出度
                        if (--outDegree[ni][nj] == 0) {
                            q.emplace(ni, nj);
                        }
                    }
                }
            }
        }

        return pathLength;
    }
};
```
