# 289. Game of Life

### Description

According to Wikipedia's article: "The Game of Life, also known simply as Life, is a cellular automaton devised by the British mathematician John Horton Conway in 1970."

The board is made up of an m x n grid of cells, where each cell has an initial state: live (represented by a 1) or dead (represented by a 0). Each cell interacts with its eight neighbors (horizontal, vertical, diagonal) using the following four rules (taken from the above Wikipedia article):

1. Any live cell with fewer than two live neighbors dies as if caused by under-population.
2. Any live cell with two or three live neighbors lives on to the next generation.
3. Any live cell with more than three live neighbors dies, as if by over-population.
4. Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
The next state of the board is determined by applying the above rules simultaneously to every cell in the current state of the m x n grid board. In this process, births and deaths occur simultaneously.

Given the current state of the board, update the board to reflect its next state.

Note that you do not need to return anything.

### Example 

###### Example I:

![](./grid1.jpg)

```
Input: board = [[0,1,0],[0,0,1],[1,1,1],[0,0,0]]
Output: [[0,0,0],[1,0,1],[0,1,1],[0,1,0]]
```

###### Example II

![](./grid2.jpg)

```
Input: board = [[1,1],[1,0]]
Output: [[1,1],[1,1]]
```

### Solution

遍历统计

```c++
class Solution {
public:
    void gameOfLife(vector<vector<int>>& board) {
        int m = board.size();
        int n = board[0].size();
        vector<vector<int>> copy;
        for (int i = 0; i < m; i++) {
            vector<int> current;
            for (int j = 0; j < n; j++) {
                current.push_back(board[i][j]);
            }
            copy.push_back(current);
        }

        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                board[i][j] = live(copy, i, j);
            }
        }
    }

private:
    int live(vector<vector<int>>& copy, int i, int j) {
        int m = copy.size();
        int n = copy[0].size();
        int status = copy[i][j];
        int live_count = 0;
        if (i - 1 > -1 && j - 1 > -1) live_count += copy[i - 1][j - 1];
        if (i - 1 > -1) live_count += copy[i - 1][j];
        if (i - 1 > -1 && j + 1 < n) live_count += copy[i - 1][j + 1];
        if (j + 1 < n) live_count += copy[i][j + 1];
        if (i + 1 < m && j + 1 < n) live_count += copy[i + 1][j + 1];
        if (i + 1 < m) live_count += copy[i + 1][j];
        if (i + 1 < m && j - 1 > -1) live_count += copy[i + 1][j - 1];
        if (j - 1 > -1) live_count += copy[i][j - 1];

        if (status == 1) {
            if (live_count < 2 || live_count > 3) return 0;
            else return 1;
        } else {
            if (live_count == 3) return 1;
            else return 0;
        }
        return status;
    }
};
```
