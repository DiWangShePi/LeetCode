# 211. Design Add and Search Words Data Structure

### Description

Design a data structure that supports adding new words and finding if a string matches any previously added string.

Implement the WordDictionary class:

- WordDictionary() Initializes the object.
- void addWord(word) Adds word to the data structure, it can be matched later.
- bool search(word) Returns true if there is any string in the data structure that matches word or false otherwise. word may contain dots '.' where dots can be matched with any letter.

### Solution

前缀树，`.`代表通配符

### Implementation

###### c++

```c++
#include <unordered_map>
#include <string>
#include <vector>

using namespace std;

class Node {
public:
    char c_char;
    unordered_map<char, Node*> edges;
    bool isEnd;

    Node(char c) : c_char(c), isEnd(false) {}
};

class WordDictionary {
private: 
    Node* root;

    bool dfs(Node* node, const string& word, int index) {
        if (index == word.size()) {
            return node->isEnd;
        }

        char c = word[index];
        if (c == '.') {
            for (auto& [key, nextNode] : node->edges) {
                if (dfs(nextNode, word, index + 1)) {
                    return true;
                }
            }
            return false;
        } else {
            if (node->edges.count(c) == 0) return false;
            return dfs(node->edges[c], word, index + 1);
        }
    }

public:
    WordDictionary() {
        root = new Node('0');
    }
    
    void addWord(string word) {
        Node* current = root;
        for (char c : word) {
            if (current->edges.count(c) == 0) { 
                current->edges[c] = new Node(c);
            }
            current = current->edges[c];
        }
        current->isEnd = true;
    }
    
    bool search(string word) {
        return dfs(root, word, 0);
    }
};

/**
 * Your WordDictionary object will be instantiated and called as such:
 * WordDictionary* obj = new WordDictionary();
 * obj->addWord(word);
 * bool param_2 = obj->search(word);
 */
```
