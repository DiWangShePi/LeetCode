# 37. Sudoku Solver

### 题目描述

输入一个九乘九的方格，完成数独游戏。

一个合法的方格需要满足如下条件：

1. 每一行数字1-9不能重复。
2. 每一列数字1-9不能重复。
3. 每一个3乘3的小格中，数字1到9不能重复。（0-2,3-5,6-8）

**示例：**

```
Input: board = [["5","3",".",".","7",".",".",".","."],["6",".",".","1","9","5",".",".","."],[".","9","8",".",".",".",".","6","."],["8",".",".",".","6",".",".",".","3"],["4",".",".","8",".","3",".",".","1"],["7",".",".",".","2",".",".",".","6"],[".","6",".",".",".",".","2","8","."],[".",".",".","4","1","9",".",".","5"],[".",".",".",".","8",".",".","7","9"]]
Output: [["5","3","4","6","7","8","9","1","2"],["6","7","2","1","9","5","3","4","8"],["1","9","8","3","4","2","5","6","7"],["8","5","9","7","6","1","4","2","3"],["4","2","6","8","5","3","7","9","1"],["7","1","3","9","2","4","8","5","6"],["9","6","1","5","3","7","2","8","4"],["2","8","7","4","1","9","6","3","5"],["3","4","5","2","8","6","1","7","9"]]
```

### 题目解析

回溯

### 代码实现

###### c++

```c++
class Solution {
public:
    bool row[9][10], col[9][10], sq[9][10];

    bool isValid(int i, int j, int num) {
        int k = (i / 3) * 3 + (j / 3);
        return !row[i][num] && !col[j][num] && !sq[k][num];
    }

    void visit(int i, int j, int num) {
        int k = (i / 3) * 3 + (j / 3);
        row[i][num] = col[j][num] = sq[k][num] = true;
    }

    void unvisit(int i, int j, int num) {
        int k = (i / 3) * 3 + (j / 3);
        row[i][num] = col[j][num] = sq[k][num] = false;
    }

    bool Backtrack(vector<vector<char>>& board, int i, int j) {
        if (i == 9) return true;
        if (j == 9) return Backtrack(board, i + 1, 0);
        if (board[i][j] != '.') return Backtrack(board, i, j + 1);

        for (int num = 1; num <= 9; num++) {
            if (isValid(i, j, num)) {
                board[i][j] = num + '0';
                visit(i, j, num);
                if (Backtrack(board, i, j + 1)) return true;
                board[i][j] = '.';
                unvisit(i, j, num);
            }
        }
        return false;
    }

    void solveSudoku(vector<vector<char>>& board) {
        memset(row, false, sizeof(row));
        memset(col, false, sizeof(col));
        memset(sq, false, sizeof(sq));
        for (int i = 0; i < 9; i++) {
            for (int j = 0; j < 9; j++) {
                if (board[i][j] != '.') {
                    int num = board[i][j] - '0';
                    visit(i, j, num);
                }
            }
        }
        Backtrack(board, 0, 0);
    }
};
```