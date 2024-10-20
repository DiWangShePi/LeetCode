# 79. Word Search

### 题目描述

给定一个由m x n个字符组成的网格和一个字符串单词，如果单词存在于网格中，则返回true。单词可以由顺序相邻单元的字母构成，其中相邻单元是水平或垂直相邻的。同一个字母单元格不能重复使用一次。

**示例：**

```
Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCCED"
Output: true
```

```
Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "SEE"
Output: true
```

```
Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCB"
Output: false
```

### 题目解析

遍历数组，每次找到第一个字符即进行周围的逐次遍历，满足条件则返回为真。

### 代码实现

###### c++

```c++
class Solution {
public:
    bool exist(vector<vector<char>>& board, string word) {
        int m = board.size();
        int n = board[0].size();
        vector<vector<bool>> visited(m, vector<bool>(n, false));
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                if (board[i][j] == word[0]) {
                    visited[i][j] = true;
                    if (search(board, visited, word, 1, i, j)) return true;
                    visited[i][j] = false;
                }
            }
        }
        return false;
    }

    bool search(vector<vector<char>>& board, vector<vector<bool>>& visited, string word, int index, int i, int j) {
        if (index == word.size()) return true;

        if (i - 1 > -1 && board[i-1][j] == word[index] && !visited[i-1][j]) {
            visited[i-1][j] = true;
            if (search(board, visited, word, index+1, i-1, j)) return true;
            visited[i-1][j] = false;
        }
        if (i + 1 < board.size() && board[i+1][j] == word[index] && !visited[i+1][j]) {
            visited[i+1][j] = true;
            if (search(board, visited, word, index+1, i+1, j)) return true;
            visited[i+1][j] = false;
        }
        if (j - 1 > -1 && board[i][j-1] == word[index] && !visited[i][j-1]) {
            visited[i][j-1] = true;
            if (search(board, visited, word, index+1, i, j-1)) return true;
            visited[i][j-1] = false;
        }
        if (j + 1 < board[0].size() && board[i][j+1] == word[index] && !visited[i][j+1]) {
            visited[i][j+1] = true;
            if (search(board, visited, word, index+1, i, j+1)) return true;
            visited[i][j+1] = false;
        }
        return false;
    }
};
```

look at this impl:

```c++
class Solution {

    const int directions[4][2] = {
        {0, 1},
        {1, 0},
        {0, -1},
        {-1, 0}
    };

    inline bool isInRange(int x, int y, const vector<vector<char>> &board) const {
        const int n = board.size();
        const int m = board.front().size();

        return 0 <= x && x < n && 0 <= y && y < m;
    }

    bool search(int x, int y, int pos, vector<vector<bool>> taken, const vector<vector<char>> &board, const string &word) {
        if(pos >= word.size()) {
            return true;
        }

        for(auto [dx, dy]: directions) {
            auto i = dx + x;
            auto j = dy + y;
            if(isInRange(i, j, board) && !taken[i][j] && board[i][j] == word[pos]) {
                taken[i][j] = true;
                if(search(i, j, pos + 1, taken, board, word)) {
                    return true;
                }
                taken[i][j] = false;
            }
        }
        return false;
    }

public:
    inline bool exist(const vector<vector<char>> &board, string word) {
        const int n = board.size();
        const int m = board.front().size();

        vector<int> cnt(128, 0);
        for(int i = 0; i < n; i++) {
            for(int j = 0; j < m; j++) {
                cnt[board[i][j]]++;
            }
        }

        for(auto c: word) {
            if(cnt[c] == 0) {
                return false;
            }
            cnt[c]--;
        }

        for(auto c: word) cnt[c]++; // undo the subtraction

        int prefixFreq = 0;
        int suffixFreq = 0;
        for(int i = 0, j = word.size()-1; i < j; i++, j--) {
            prefixFreq += cnt[word[i]];
            suffixFreq += cnt[word[j]];
        }

        if(suffixFreq < prefixFreq) { 
            // suffix chars has less freq in the matrix. 
            // so, reversing will actually reduce the search space
            reverse(word.begin(), word.end());
        }

        vector<vector<bool>> taken(n, vector<bool>(m, false));
        for(int i = 0; i < n; i++) {
            for(int j = 0; j < m; j++) {
                taken[i][j] = true;
                if(board[i][j] == word[0] && search(i, j, 1, taken, board, word)) {
                    return true;
                }
                taken[i][j] = false;
            }
        }
        return false;
    }
};
```