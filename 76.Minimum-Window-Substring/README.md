# 76. Minimum Window Substring

### 题目描述

给定两个字符串 s 和 t，长度分别为 m 和 n，返回 s 的最小窗口子字符串，使得 t 中的每个字符（包括重复字符）都包含在窗口中。如果没有这样的子字符串，则返回空字符串“”。

我们生成的测试用例确保答案是唯一的。

**示例：**

```
Input: s = "ADOBECODEBANC", t = "ABC"
Output: "BANC"
Explanation: The minimum window substring "BANC" includes 'A', 'B', and 'C' from string t.
```

```
Input: s = "a", t = "a"
Output: "a"
Explanation: The entire string s is the minimum window.
```

```
Input: s = "a", t = "aa"
Output: ""
Explanation: Both 'a's from t must be included in the window.
Since the largest window of s only has one 'a', return empty string.
```

### 题目解析

我们并不要求t中字符的顺序在s中重现，重要的是每个字符出现的个数。
因此，我们可以先遍历t，确认所有字符出现过的次数。

接下来，遍历字符串s并维护左指针，记录使当前子字符串满足要求的最早的位置。
遍历的过程中不断更新这个位置。所遇到的最小的即为答案。

### 代码实现

###### c++

```c++
class Solution {
public:
    string minWindow(string s, string t) {
        if (s.length() < t.length()) {
            return "";
        }

        unordered_map<char, int> charCount;
        for (char ch : t) {
            charCount[ch]++;
        }

        int targetCharsRemaining = t.length();
        int minWindow[2] = {0, INT_MAX};
        int startIndex = 0;

        for (int endIndex = 0; endIndex < s.length(); endIndex++) {
            char ch = s[endIndex];
            if (charCount.find(ch) != charCount.end() && charCount[ch] > 0) {
                targetCharsRemaining--;
            }
            charCount[ch]--;

            if (targetCharsRemaining == 0) {
                while (true) {
                    char charAtStart = s[startIndex];
                    if (charCount.find(charAtStart) != charCount.end() && charCount[charAtStart] == 0) {
                        break;
                    }
                    charCount[charAtStart]++;
                    startIndex++;
                }

                if (endIndex - startIndex < minWindow[1] - minWindow[0]) {
                    minWindow[0] = startIndex;
                    minWindow[1] = endIndex;
                }

                charCount[s[startIndex]]++;
                targetCharsRemaining++;
                startIndex++;
            }
        }

        return minWindow[1] >= s.length() ? "" : s.substr(minWindow[0], minWindow[1] - minWindow[0] + 1);        
    }
};
```
