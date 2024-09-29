# 58. Length of Last Word

### 题目描述

给定一个包含单词和空格的字符串，返回该字符串中最后一个单词的长度。

单词中不包含空格

**示例：**

```
Input: s = "Hello World"
Output: 5
Explanation: The last word is "World" with length 5.
```

```
Input: s = "   fly me   to   the moon  "
Output: 4
Explanation: The last word is "moon" with length 4.
```

```
Input: s = "luffy is still joyboy"
Output: 6
Explanation: The last word is "joyboy" with length 6.
```

### 题目解析

1. 将字符串按照空格分割成列表，返回列表最后一个元素的长度。

2. 从后往前遍历字符串，忽略掉空格，从第一个单词开始计数，当再次遇到空格时停止。

### 代码实现

###### c++

```c++
#include <iostream>
#include <sstream>
#include <vector>
#include <string>

class Solution {
public:
    int lengthOfLastWord(string s) {
        std::vector<std::string> result;
        std::istringstream stream(s);
        std::string word;

        while (stream >> word) {
            result.push_back(word);
        }

        return result[result.size() - 1].length();
    }
};
```