# 5. Longest Palindromic Substring

### 题目描述

给定一个字符串，要求这个字符串当中最长的回文串。

**示例1:**

```
Input: "babad"
Output: "bab"
Note: "aba" is also a valid answer.
```

**示例2:**
```
Input: "cbbd"
Output: "bb"
```

### 题目解析

参考马拉车算法：https://en.wikipedia.org/wiki/Longest_palindromic_substring#Manacher's_algorithm


### 代码实现

###### c++

```c++
class Solution {
public:
    std::string longestPalindrome(std::string s) {
        if (s.length() <= 1) {
            return s;
        }
        
        int maxLen = 1;
        std::string maxStr = s.substr(0, 1);
        s = "#" + std::regex_replace(s, std::regex(""), "#") + "#";
        std::vector<int> array(s.length(), 0);
        int center = 0;
        int right = 0;
        
        for (int i = 0; i < s.length(); ++i) {
            if (i < right) {
                array[i] = std::min(right - i, array[2 * center - i]);
            }
            
            while (i - array[i] - 1 >= 0 && i + array[i] + 1 < s.length() && s[i - array[i] - 1] == s[i + array[i] + 1]) {
                array[i]++;
            }
            
            if (i + array[i] > right) {
                center = i;
                right = i + array[i];
            }
            
            if (array[i] > maxLen) {
                maxLen = array[i];
                maxStr = s.substr(i - array[i], 2 * array[i] + 1);
                maxStr.erase(std::remove(maxStr.begin(), maxStr.end(), '#'), maxStr.end());
            }
        }
        
        return maxStr;
    }
};
```