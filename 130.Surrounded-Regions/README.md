# 130. Surrounded Regions

### Description

You are given an m x n matrix board containing letters 'X' and 'O', capture regions that are surrounded:

- Connect: A cell is connected to adjacent cells horizontally or vertically.
- Region: To form a region connect every 'O' cell.
- Surround: The region is surrounded with 'X' cells if you can connect the region with 'X' cells and none of the region cells are on the edge of the board.
A surrounded region is captured by replacing all 'O's with 'X's in the input matrix board.

### Solution

如果只是将O换成X，那就是遍历的问题。但需要考虑是相邻的，所以需要深度搜索。由于还需要不是边界，因此可以遍历边界，排除所有错误答案，再遍历解决正确答案。

### Implementation

###### c++

```c++
class Solution {
public:
    void solve(vector<vector<char>>& board) {
        if (board.empty() || board[0].empty()) return;

        int m = board.size();
        int n = board[0].size();

        auto dfs = [&](int x, int y, auto&& dfs_ref) -> void {
            if (x < 0 || y < 0 || x >= m || y >= n || board[x][y] != 'O') return;
            board[x][y] = 'T'; 
            dfs_ref(x + 1, y, dfs_ref);
            dfs_ref(x - 1, y, dfs_ref);
            dfs_ref(x, y + 1, dfs_ref);
            dfs_ref(x, y - 1, dfs_ref);
        };

        for (int i = 0; i < m; ++i) {
            if (board[i][0] == 'O') dfs(i, 0, dfs);
            if (board[i][n - 1] == 'O') dfs(i, n - 1, dfs);
        }
        for (int j = 0; j < n; ++j) {
            if (board[0][j] == 'O') dfs(0, j, dfs);
            if (board[m - 1][j] == 'O') dfs(m - 1, j, dfs);
        }

        for (int i = 0; i < m; ++i) {
            for (int j = 0; j < n; ++j) {
                if (board[i][j] == 'O') {
                    board[i][j] = 'X'; 
                } else if (board[i][j] == 'T') {
                    board[i][j] = 'O'; 
                }
            }
        }
    }
};
```
