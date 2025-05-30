# 387. First Unique Character in a String

### Description

Given a string s, find the first non-repeating character in it and return its index. If it does not exist, return -1.

### Example 

###### Example I

```
Input: s = "leetcode"
Output: 0

Explanation:
The character 'l' at index 0 is the first character that does not occur at any other index.
```

###### Example II

```
Input: s = "loveleetcode"
Output: 2
```

###### Example III

```
Input: s = "aabb"
Output: -1
```

### Solution

遍历一遍计数，遍历第二遍检查计数

```c++
class Solution {
public:
    int firstUniqChar(string s) {
        vector<int> letter(26, 0);
        for (int i = 0; i < s.size(); i++) letter[s[i] - 'a']++;
        for (int i = 0; i < s.size(); i++) if (letter[s[i] - 'a'] == 1) return i;
        return -1;
    }
};
```