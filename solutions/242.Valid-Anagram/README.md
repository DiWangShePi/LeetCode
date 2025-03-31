# 242. Valid Anagram

### Description

Given two strings s and t, return true if t is an anagram of s, and false otherwise.

### Solution

用字典遍历

```c++
class Solution {
public:
    bool isAnagram(string s, string t) {
        if (s.size() != t.size()) return false;

        unordered_map<char, int> dict;
        for (int i = 0; i < s.size(); i++) {
            dict[s[i]]++;
            dict[t[i]]--;
        }
        for (auto x : dict) {
            if (x.second) return false;
        }
        return true;
    }
};
```
