# 467. Unique Substrings in Wraparound String

### Description

We define the string base to be the infinite wraparound string of "abcdefghijklmnopqrstuvwxyz", so base will look like this:

- "...zabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzabcd....".
Given a string s, return the number of unique non-empty substrings of s are present in base.

### Example 

###### Example I

> Input: s = "a"
> Output: 1
> Explanation: Only the substring "a" of s is in base.

###### Example II

> Input: s = "cac"
> Output: 2
> Explanation: There are two substrings ("a", "c") of s in base.

###### Example III

> Input: s = "zab"
> Output: 6
> Explanation: There are six substrings ("z", "a", "b", "za", "ab", and "zab") of s in base.

### Solution

> 暴力解也是解

```c++
class Solution {
public:
    int findSubstringInWraproundString(string s) {
        unordered_map<string, int> dict;
        for (int i = 0; i < s.size(); i++) dict[s.substr(i, 1)] = 1;

        int m = 0, n = 0;
        for (int i = 0; i < s.size(); i++) {
            for (int j = i + 1; j < s.size(); j++) {
                if (dict.count(s.substr(i, j - i)) != 0) {
                    m = (s[j - 1] + 1 - 97) % 26;
                    n = s[j] - 97;
                    if (m == n) dict[s.substr(i, j - i + 1)];
                    else break;
                } else break;
            }
        }
        return dict.size();
    }
};
```

我们随后发现，上面的代码中有很多重复且不必要的计算：base本身是重复的，那么对于s中长度不同且以相同字符结尾的子字符串，长的一定包含短的。由此，我们定义dp[c]为字符串s中以字符c结尾的子串最长长度（base是重复的，因此我们在给定长度下有唯一的子字符串，就不需要考虑重复问题）。遍历s的过程中即可得到该值。

```c++
class Solution {
public:
    int findSubstringInWraproundString(string s) {
        vector<int> dp(26);
        int k = 0;
        for (int i = 0; i < s.size(); i++) {
            if (i && (s[i] - s[i - 1] + 26) % 26 == 1) k++;
            else k = 1;

            dp[s[i] - 'a'] = max(dp[s[i] - 'a'], k); 
        }
        return accumulate(dp.begin(), dp.end(), 0);
    }
};
```
