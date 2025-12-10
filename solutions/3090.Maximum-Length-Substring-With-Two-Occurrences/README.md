# 3090. Maximum Length Substring With Two Occurrences

**Tags:** Sliding Window, Dict

### Description

Given a string s, return the maximum length of a substring such that it contains at most two occurrences of each character.

### Example

###### Example I

> Input: s = "bcbbbcba"
> Output: 4
> Explanation:
> The following substring has a length of 4 and contains at most two occurrences of each character: "bcbbbcba".

###### Example II

> Input: s = "aaaa"
> Output: 2
> Explanation:
> The following substring has a length of 2 and contains at most two occurrences of each character: "aaaa".

### Solution

遍历字符串，用字典记录字符的出现与否。出现某个不满足条件的字符时，移动左指针至重新满足条件。记录满足条件的最长子字符串。

```c++
class Solution {
public:
    int maximumLengthSubstring(string s) {
        unordered_map<char, vector<int>> dict;

        int an = INT_MIN, left = 0, n = s.size();
        for (int i = 0; i < n; i++) {
            char c = s[i];
            if (dict.count(c) == 0) dict[c] = vector<int>{i};
            else {
                if (dict[c].size() == 1) dict[c].push_back(i);
                else {
                    vector<int> curr = dict[c];
                    dict[c] = vector<int>{curr[1], i};

                    // cout << i << endl;
                    an = max(an, i - left);
                    left = max(left, curr[0] + 1);
                }
            }
        }
        return max(an, n - left);
    }
};
```
