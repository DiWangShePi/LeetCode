# 131. Palindrome Partitioning

### Description

Given a string s, partition s such that every substring of the partition is a palindrome. Return all possible palindrome partitioning of s.

### Solution

使用深度优先搜索，考虑每一种可能的字符串子序列，检查其是否是回文的。如果是，就加入到列表中，进入下一层。如果最终能够到达底部（即包含了完整的字符串），就是一个合法的答案。

### Implementation

###### c++

```c++
class Solution {
public:
    vector<vector<string>> partition(string s) {
        vector<vector<string>> answer;
        vector<string> current;
        dfs(0, s, current, answer);
        return answer;
    }

    bool check(string s){
        int i = 0;
        int j = s.size() - 1;
        for ( ; j > i; i++, j--) {
            if (s[i] != s[j]) return false;
        }
        return true;
    }

    void dfs(int index, string& s, vector<string>& current, vector<vector<string>>& answer) {
        if (index == s.size()) {
            answer.push_back(current);
        }

        for (int i = index; i < s.size(); i++) {
            if (check(s.substr(index, i - index + 1))) {
                current.push_back(s.substr(index, i - index + 1));
                dfs(i + 1, s, current, answer);
                current.pop_back();
            }
        }
    }
};
```
