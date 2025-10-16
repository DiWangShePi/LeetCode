# 524. Longest Word in Dictionary through Deleting

**Tags:** String

### Description

Given a string s and a string array dictionary, return the longest string in the dictionary that can be formed by deleting some of the given string characters. If there is more than one possible result, return the longest word with the smallest lexicographical order. If there is no possible result, return the empty string.

### Example 

###### Example I

> Input: s = "abpcplea", dictionary = ["ale","apple","monkey","plea"]
> Output: "apple"

###### Example II

> Input: s = "abpcplea", dictionary = ["a","b","c"]
> Output: "a"

### Solution

先将给定的数组按照长度降序排列，长度相同时，按照字典序升序排列。随后遍历检查，第一个遇到的就是符合要求的。

```c++
class Solution {
public:
    string findLongestWord(string s, vector<string>& dictionary) {
        sort(dictionary.begin(), dictionary.end(), [](const string& a, const string& b) {
            if (a.size() != b.size()) return a.size() > b.size();  
            else return a < b;  
        });

        for (const string& t : dictionary) {
            if (check(s, t)) return t;
        }
        return "";
    }

private:
    bool check(const string& s, const string& t) {
        int p_s = 0, p_t = 0;
        while (p_s < s.size() && p_t < t.size()) {
            if (s[p_s] == t[p_t]) p_t++;
            p_s++;
        }
        return p_t == t.size();
    }
};
```
