# 567. Permutation in String

**Tags:** Sliding Window, Dict

### Description

Given two strings s1 and s2, return true if s2 contains a permutation of s1, or false otherwise.

In other words, return true if one of s1's permutations is the substring of s2.

### Example

###### Example I

> Input: s1 = "ab", s2 = "eidbaooo"
> Output: true
> Explanation: s2 contains one permutation of s1 ("ba").

###### Example II

> Input: s1 = "ab", s2 = "eidboaoo"
> Output: false

### Solution

字典记录指定长度的字符串，出现的所有字符次数。滑动窗口边界处的字符对记录值产生变化，与 s1 构成的进行比较。

```c++
class Solution {
public:
    bool checkInclusion(string s1, string s2) {
        if (s1.size() > s2.size()) return false;

        vector<int> s1_dict(26, 0), s2_dict(26, 0);
        for (char c : s1) s1_dict[c - 'a']++;

        int i = 0; 
        for ( ; i < s1.size(); i++) s2_dict[s2[i] - 'a']++;
        if (check(s1_dict, s2_dict)) return true;
        for ( ; i < s2.size(); i++) {
            s2_dict[s2[i] - 'a']++;
            s2_dict[s2[i - s1.size()] - 'a']--;
            if (check(s1_dict, s2_dict)) return true;
        }
        return false;
    }

private:
    bool check(vector<int>& dict1, vector<int>& dict2) {
        for (int i = 0; i < 26; i++) {
            if (dict1[i] != dict2[i]) return false;
        }
        return true;
    }
};
```
