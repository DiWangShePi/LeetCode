# 522. Longest Uncommon Subsequence II

**Tags:** String

### Description

iven an array of strings strs, return the length of the longest uncommon subsequence between them. If the longest uncommon subsequence does not exist, return -1.

An uncommon subsequence between an array of strings is a string that is a subsequence of one string but not the others.

A subsequence of a string s is a string that can be obtained after deleting any number of characters from s.

- For example, "abc" is a subsequence of "aebdc" because you can delete the underlined characters in "aebdc" to get "abc". Other subsequences of "aebdc" include "aebdc", "aeb", and "" (empty string).

### Example 

###### Example I

> Input: strs = ["aba","cdc","eae"]
> Output: 3

###### Example II

> Input: strs = ["aaa","aaa","aa"]
> Output: -1

### Solution

原理与上一题一样：只要有一个字符不一样，整个字符串都可以是不一样的。

因此我们可以枚举所有可能的组合，考虑一个字符串是否存在与另一个字符串不同的字符。记录最长的哪一个即可。

```c++
class Solution {
public:
    int findLUSlength(vector<string>& strs) {
        int n = strs.size();
        int ans = -1;
        for (int i = 0; i < n; ++i) {
            bool check = true;
            for (int j = 0; j < n; ++j) {
                if (i != j && is_subseq(strs[i], strs[j])) {
                    check = false;
                    break;
                }
            }
            int l = strs[i].size();
            if (check) ans = max(ans, l);
        }
        return ans;
    }

private:
    bool is_subseq(const string& s, const string& t) {
        int pt_s = 0, pt_t = 0;
        while (pt_s < s.size() && pt_t < t.size()) {
            if (s[pt_s] == t[pt_t]) {
                ++pt_s;
            }
            ++pt_t;
        }
        return pt_s == s.size();
    }
};
```

### Related "Works"

1. LeetCode 521. Longest Uncommon Subsequence I
