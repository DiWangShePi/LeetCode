# 290. Word Pattern

### Description

Given a pattern and a string s, find if s follows the same pattern.

Here follow means a full match, such that there is a bijection between a letter in pattern and a non-empty word in s. Specifically:

Each letter in pattern maps to exactly one unique word in s.
Each unique word in s maps to exactly one letter in pattern.
No two letters map to the same word, and no two words map to the same letter.

### Example

###### Example I:

```
Input: pattern = "abba", s = "dog cat cat dog"

Output: true

Explanation:

The bijection can be established as:

'a' maps to "dog".
'b' maps to "cat".
```

###### Example II

```
Input: pattern = "abba", s = "dog cat cat fish"

Output: false
```

###### Example III

```
Input: pattern = "aaaa", s = "dog cat cat dog"

Output: false
```

### Solution

双向映射，类比第205题。

```c++
class Solution {
public:
    bool wordPattern(string pattern, string s) {
        istringstream iss(s);
        vector<string> words;
        string word;
        while (iss >> word) words.push_back(word);
        if (words.size() != pattern.size()) return false;

        unordered_map<char, string> forward;
        unordered_map<string, char> reverse;
        for (int i = 0; i < words.size(); i++) {
            char current_c = pattern[i];
            string current_s = words[i];

            if (forward.count(current_c) != 0 && forward[current_c] != current_s) return false;
            if (reverse.count(current_s) != 0 && reverse[current_s] != current_c) return false;

            forward[current_c] = current_s;
            reverse[current_s] = current_c;
        }
        return true;
    }
};
```
