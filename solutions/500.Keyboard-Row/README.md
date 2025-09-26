# 500. Keyboard Row

### Description

Given an array of strings words, return the words that can be typed using letters of the alphabet on only one row of American keyboard like the image below.

Note that the strings are case-insensitive, both lowercased and uppercased of the same letter are treated as if they are at the same row.

In the American keyboard:

- the first row consists of the characters "qwertyuiop",
- the second row consists of the characters "asdfghjkl", and
- the third row consists of the characters "zxcvbnm".

![](./keyboard.png)

### Example 

###### Example I

> Input: words = ["Hello","Alaska","Dad","Peace"]
> Output: ["Alaska","Dad"]
> Explanation:
> Both "a" and "A" are in the 2nd row of the American keyboard due to case insensitivity.

###### Example II

> Input: words = ["omk"]
> Output: []

###### Example III

> Input: words = ["adsdf","sfd"]
> Output: ["adsdf","sfd"]

### Solution

用字典记录每一个字符属于哪一行

```c++
class Solution {
public:
    vector<string> findWords(vector<string>& words) {
        unordered_map<char, int> dict {
            {'q', 1}, {'w', 1}, {'e', 1}, {'r', 1}, {'t', 1}, {'y', 1}, {'u', 1}, {'i', 1}, {'o', 1}, {'p', 1},
            {'a', 2}, {'s', 2}, {'d', 2}, {'f', 2}, {'g', 2}, {'h', 2}, {'j', 2}, {'k', 2}, {'l', 2},
            {'z', 3}, {'x', 3}, {'c', 3}, {'v', 3}, {'b', 3}, {'n', 3}, {'m', 3}
        };

        vector<string> an;
        for (string& w : words) {
            if (check(w, dict)) an.push_back(w);
        }
        return an;
    }

private:
    bool check(string& w, unordered_map<char, int> dict) {
        int line = dict[tolower(w[0])];
        for (char c : w) {
            if (dict[tolower(c)] != line) return false;
        }
        return true;
    }
};
```
