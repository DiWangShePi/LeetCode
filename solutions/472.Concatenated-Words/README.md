# 472. Concatenated Words

**Tags:** Set,   

### Description

Given an array of strings words (without duplicates), return all the concatenated words in the given list of words.

A concatenated word is defined as a string that is comprised entirely of at least two shorter words (not necessarily distinct) in the given array.

### Example

###### Example I

> Input: words = ["cat","cats","catsdogcats","dog","dogcatsdog","hippopotamuses","rat","ratcatdogcat"]
> Output: ["catsdogcats","dogcatsdog","ratcatdogcat"]
> Explanation: "catsdogcats" can be concatenated by "cats", "dog" and "cats"; 
> "dogcatsdog" can be concatenated by "dog", "cats" and "dog"; 
> "ratcatdogcat" can be concatenated by "rat", "cat", "dog" and "cat".

###### Example II

> Input: words = ["cat","dog","catdog"]
> Output: ["catdog"]

### Solution

我用的方法是暴力：先将所有的字符串存入集合中，然后对每一个字符串进行O(N^2)的遍历式检索，如果能被其他字符串构成，就加入到答案中。

```c++
class Solution {
public:
    vector<string> findAllConcatenatedWordsInADict(vector<string>& words) {
        unordered_set<string> dict(words.begin(), words.end());
        vector<string> result;
        
        for (const string& word : words) {
            if (word.empty()) continue;
            if (canForm(word, dict)) {
                result.push_back(word);
            }
        }
        
        return result;
    }
    
private:
    bool canForm(const string& word, const unordered_set<string>& dict) {
        vector<bool> dp(word.size() + 1, false);
        dp[0] = true;
        
        for (int i = 1; i <= word.size(); i++) {
            for (int j = (i == word.size() ? 1 : 0); j < i; j++) {
                if (dp[j] && dict.count(word.substr(j, i - j))) {
                    dp[i] = true;
                    break;
                }
            }
        }
        
        return dp[word.size()];
    }
};
```

不过，这一题的标准解法应该是用字典树来完成。
我们首先将字符串按照长度排序，这样可以确保在处理指定字符串时，所有可能将其构成的字字符串都已经存在于字典树中了。
因此我们可以遍历字符串，如果当前字符串可以被多于一个的字字符串构成，就将其加入答案中。反之将其加入字典树中。

检查一个字符串能否被其他字符串构成，就是逐个遍历字符。
- 如果当前字符不存在于字典树中，说明失败了，返回
- 如果当前字符存在于字典树中，检查当前字符是否是结尾
- - 如果是，尝试拆分并进入下一层
- 如果不是，继续

记忆化搜索可以记录当前位置是否能够满足条件，使得程序在尝试不同的拆分方案，但走到相同的一步时避免重复搜索浪费的时间。

```c++
struct Trie {
    bool isEnd;
    vector<Trie *> children;
    Trie() {
        this->children = vector<Trie *>(26, nullptr);
        this->isEnd = false;
    }
};

class Solution {
public:
    Trie * trie = new Trie();

    vector<string> findAllConcatenatedWordsInADict(vector<string>& words) {
        vector<string> ans;
        sort(words.begin(), words.end(), [&](const string & a, const string & b){
            return a.size() < b.size(); 
        });
        for (int i = 0; i < words.size(); i++) {
            string word = words[i];
            if (word.size() == 0) {
                continue;
            }
            vector<int> visited(word.size(), 0);
            if (dfs(word, 0, visited)) {
                ans.emplace_back(word);
            } else {
                insert(word);
            }
        }
        return ans;
    }

    bool dfs(const string & word, int start, vector<int> & visited) {
        if (word.size() == start) {
            return true;
        }
        if (visited[start]) {
            return false;
        }
        visited[start] = true;
        Trie * node = trie;
        for (int i = start; i < word.size(); i++) {
            char ch = word[i];
            int index = ch - 'a';
            node = node->children[index];
            if (node == nullptr) {
                return false;
            }
            if (node->isEnd) {
                if (dfs(word, i + 1, visited)) {
                    return true;
                }
            }
        }
        return false;
    }

    void insert(const string & word) {
        Trie * node = trie;
        for (int i = 0; i < word.size(); i++) {
            char ch = word[i];
            int index = ch - 'a';
            if (node->children[index] == nullptr) {
                node->children[index] = new Trie();
            }
            node = node->children[index];
        }
        node->isEnd = true;
    }
};
```

### Related "Work"

1. LeetCode 208. Implement Trie (Prefix Tree)
