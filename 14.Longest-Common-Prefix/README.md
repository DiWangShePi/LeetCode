# 14. Longest Common Prefix

### 题目描述

编写一个函数来查找字符串数组中的最长公共前缀。

如果不存在公共前缀，返回空字符串 ""。

**示例：**

```
输入：strs = ["flower","flow","flight"]
输出："fl"
```

```
输入：strs = ["dog","racecar","car"]
输出：""
解释：输入不存在公共前缀。
```

### 题目解析

为数组中的每一个字符串创建一个指针，遍历每个字符，确认他们的前几位是否相等
特别考虑：
1. 当输入的数组长度为0时，返回空字符串
2. 预先检索数组，找到长度最短的字符串。确保后续遍历时不会越界。

### 代码实现

###### c++
```c++
#include <string>
#include <vector>

using namespace std;

class Solution {
public:
    string longestCommonPrefix(vector<string>& strs) {
        if (strs.empty()) {
            return "";
        }
        
        int minLength = strs[0].size();
        for (const string& str : strs) {
            minLength = min(minLength, (int)str.size());
        }

        std::string answer = "";
        for (int i = 0; i < minLength; i++) {
            char currentChar = strs[0][i];
            for (int j = 1; j < strs.size(); j++) {
                if (currentChar != strs[j][i]) {
                    return answer;
                }
            }
            answer += currentChar;
        }
        return answer;
    }
};
```