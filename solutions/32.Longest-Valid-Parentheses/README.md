# 32. Longest Valid Parentheses

### 题目描述

给定一个仅包含字符“(”和“)”的字符串，返回最长有效（格式正确）括号子字符串的长度

**示例：**

```
Input: s = "(()"
Output: 2
Explanation: The longest valid parentheses substring is "()".
```

```
Input: s = ")()())"
Output: 4
Explanation: The longest valid parentheses substring is "()()".
```

```
Input: s = ""
Output: 0
```

### 题目解析

###### 暴力解法

有效的括号长度必定为偶数。从输入字符串最长的偶数开始，逐步遍历这个长度的子字符串，检查该字符串是否合法。若全部不合法则减少偶数，直至找到第一个符合要求的子字符串。

###### 正向逆向结合法

维护三个变量，left，right，max_length，分别代表遇到的左括号长度，右括号长度和最大长度。
遇到一个左括号则left加一，遇到一个右括号则left加一。当两者相等时，即为一个合法字符串，用left加right更新max_length。
从左向右遍历时，若遇到右括号大于左括号，即代表前面的字符串无法与后面字符串组成合法字符串，将left和right的值清零。
为了保证左括号大于右括号不会带来影响，我们需要在从左往右遍历结束后再从右往左遍历一遍。


### 代码实现

###### c++

```c++
class Solution {
public:
    int longestValidParentheses(string s) {
        stack<int> st;
        st.push(-1);
        int max_len = 0;

        for (int i = 0; i < s.length(); i++) {
            if (s[i] == '(') {
                st.push(i);
            } else {
                st.pop();
                if (st.empty()) {
                    st.push(i);
                } else {
                    max_len = max(max_len, i - st.top());
                }
            }
        }

        return max_len;        
    }
};
```