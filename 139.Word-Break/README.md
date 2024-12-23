# 139. Word Break

### Description

Given a string s and a dictionary of strings wordDict, return true if s can be segmented into a space-separated sequence of one or more dictionary words.

Note that the same word in the dictionary may be reused multiple times in the segmentation.

### Solution

采用动态规划的方法，定义dp[i] = dp[j] + check(j, i-1)，即字符串s[0..i]能否被分割取决于字符串s[0..j-1]能否分割和s[j..i-1]是否出现在字典中。

随后遍历枚举j检查即可，返回dp表最后一个元素的值。

### Implementation

###### c++

```c++
class Solution {
public:
    bool wordBreak(string s, vector<string>& wordDict) {
        auto wordDictSet = unordered_set<string>();
        for (auto word : wordDict) {
            wordDictSet.insert(word);
        }

        auto dp = vector<bool>(s.size() + 1);
        dp[0] = true;

        for (int i = 1; i < s.size() + 1; i++) {
            for (int j = 0; j < i; j++) {
                if (dp[j] && wordDictSet.find(s.substr(j, i - j)) != wordDictSet.end()) {
                    dp[i] = true;
                    break;
                }
            }
        }
        return dp[s.size()];
    }
};
```
