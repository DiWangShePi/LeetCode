# 208. Implement Trie (Prefix Tree)

### Description

A trie (pronounced as "try") or prefix tree is a tree data structure used to efficiently store and retrieve keys in a dataset of strings. There are various applications of this data structure, such as autocomplete and spellchecker.

Implement the Trie class:

- Trie() Initializes the trie object.
- void insert(String word) Inserts the string word into the trie.
- boolean search(String word) Returns true if the string word is in the trie (i.e., was inserted before), and false otherwise.
- boolean startsWith(String prefix) Returns true if there is a previously inserted string word that has the prefix prefix, and false otherwise.

### Solution

按要求实现即可

### Implementation

###### c++

```c++
class Node {
public:
    char c_char;
    unordered_map<char, Node*> edges;
    bool is_end;

    Node(char c) : c_char(c), is_end(false) {}
};

class Trie {
private:
    Node* root;

    bool find(const string& word, bool check_end) {
        Node* current = root;
        for (char c : word) {
            if (current->edges.find(c) == current->edges.end()) {
                return false;
            }
            current = current->edges[c];
        }
        return check_end ? current->is_end : true;
    }

public:
    Trie() {
        root = new Node('0');
    }
    
    void insert(string word) {
        Node* current = root;
        for (char c : word) {
            if (current->edges.find(c) == current->edges.end()) {
                current->edges[c] = new Node(c);
            }
            current = current->edges[c];
        }
        current->is_end = true;
    }
    
    bool search(string word) {
        return find(word, true);
    }
    
    bool startsWith(string prefix) {
        return find(prefix, false);
    }
};
```
