# 212. Word Search II

### Description

Given an m x n board of characters and a list of strings words, return all words on the board.

Each word must be constructed from letters of sequentially adjacent cells, where adjacent cells are horizontally or vertically neighboring. The same letter cell may not be used more than once in a word.

### Solution

字典树加深度优先搜索

### Implementation

###### c++

```c++
struct TrieNode {
    TrieNode* children[26] = {nullptr};
    string word = "";
};

class Trie {
public:
    TrieNode* root;
    
    Trie() {
        root = new TrieNode();
    }

    void insert(const string& word) {
        TrieNode* node = root;
        for (char c : word) {
            int index = c - 'a';
            if (!node->children[index]) {
                node->children[index] = new TrieNode();
            }
            node = node->children[index];
        }
        node->word = word;
    }
};

class Solution {
private:
    vector<string> result;
    int rows, cols;
    vector<vector<bool>> visited;
    vector<pair<int, int>> directions = {{0, 1}, {1, 0}, {0, -1}, {-1, 0}};

    void dfs(vector<vector<char>>& board, int i, int j, TrieNode* node) {
        char c = board[i][j];
        int index = c - 'a';
        if (!node->children[index]) return;

        TrieNode* nextNode = node->children[index];
        if (!nextNode->word.empty()) {
            result.push_back(nextNode->word);
            nextNode->word = "";
        }

        visited[i][j] = true;
        for (auto [dx, dy] : directions) {
            int ni = i + dx, nj = j + dy;
            if (ni >= 0 && ni < rows && nj >= 0 && nj < cols && !visited[ni][nj]) {
                dfs(board, ni, nj, nextNode);
            }
        }
        visited[i][j] = false;
    }

public:
    vector<string> findWords(vector<vector<char>>& board, vector<string>& words) {
        Trie trie;
        for (const string& word : words) {
            trie.insert(word);
        }

        rows = board.size();
        cols = board[0].size();
        visited.assign(rows, vector<bool>(cols, false));

        result.clear();
        for (int i = 0; i < rows; ++i) {
            for (int j = 0; j < cols; ++j) {
                dfs(board, i, j, trie.root);
            }
        }

        return result;
    }
};
```
