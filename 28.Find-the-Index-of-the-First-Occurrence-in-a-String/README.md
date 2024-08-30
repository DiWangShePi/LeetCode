# 28. Find the Index of the First Occurrence in a String

### 题目描述

给定两个字符串`needle`和`haystack`，返回`needle`在`haystack`中第一次出现的下标。

**示例：**

```
Input: haystack = "sadbutsad", needle = "sad"
Output: 0
Explanation: "sad" occurs at index 0 and 6.
The first occurrence is at index 0, so we return 0.
```

```
Input: haystack = "leetcode", needle = "leeto"
Output: -1
Explanation: "leeto" did not occur in "leetcode", so we return -1.
```

### 题目解析

遍历字符串，若当前字符与目标字符串的第一个相等，则将接下来相同长度的字符串切片进行比较。若相等即为正确值，不等则继续。

### 代码实现

###### c++

```c++
class Solution {
public:
    int strStr(string haystack, string needle) {
        if (haystack.length() < needle.length()) {
            return -1;
        }

        for (int i = 0; i < haystack.length() - needle.length() + 1; i++) {
            if (haystack[i] == needle[0]) {
                if (check(haystack, needle, i) == 0) {
                    return i;
                }
            }
        }
        return -1;
    }

private:
    int check(string haystack, string needle, int index) {
        std::string targetString = haystack.substr(index, needle.length());
        return targetString.compare(needle);
    }
};
```