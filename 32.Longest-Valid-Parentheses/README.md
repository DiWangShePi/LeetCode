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