# 51. N-Queens

### 题目描述

n皇后谜题是指在n × n个棋盘上放置n个皇后，这样就不会有两个皇后互相攻击。给定一个整数n，返回n皇后谜题的所有不同的解。你可以按任何顺序返回答案。每个解决方案都包含n个皇后位置的不同棋盘配置，其中“Q”和“。，分别表示皇后和空格。

**示例：**

```
Input: n = 4
Output: [[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
Explanation: There exist two distinct solutions to the 4-queens puzzle as shown above
```

```
Input: n = 1
Output: [["Q"]]
```

### 题目解析

深度优先搜索，遍历所有可能性并将合法的放入答案队列中。

### 代码实现

###### c++

```c++
class Solution {
public:
    vector<vector<string>> solveNQueens(int n) {
        if (n == 1) return {{"Q"}};
        if (n <= 3) return {};

        col = vector<bool> (n, false);
        dig = vector<bool> (2*n - 1, false);
        crdig = vector<bool> (2*n - 1, false);
        vector<string> board(n, string(n,'.'));

        backTrack(n, 0, board);
        return ans;
    }
private:
    vector<vector<string>> ans;
    vector<bool> col;
    vector<bool> dig;
    vector<bool> crdig;

    void backTrack(int n, int lvl, vector<string>& board) {
        if (lvl == n) {
            ans.emplace_back(board);
            return;
        }
        int toAdd = n - 1;
        for (int i = 0; i < n; i++) {
            if (!col[i] && !dig[lvl + i] && !crdig[lvl - i + toAdd]) {
                col[i] = dig[lvl + i] = crdig[lvl - i + toAdd] = true;
                board[lvl][i] = 'Q';
                backTrack(n, lvl + 1, board);
                board[lvl][i] = '.';
                col[i] = dig[lvl + i] = crdig[lvl - i + toAdd] = false;
            }
        }

    }
};
```