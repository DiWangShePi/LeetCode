# 316. Remove Duplicate Letters

### Description

Given a string s, remove duplicate letters so that every letter appears once and only once. You must make sure your result is the smallest in lexicographical order among all possible results.

### Example 

###### Example I

```
Input: s = "bcabc"
Output: "abc"
```

###### Example II

```
Input: s = "cbacdcbc"
Output: "acdb"
```

### Solution

尝试贪心算法：在每一步选择当前可以选的最小字母，同时确保后面的字母还能满足去重的条件。
具体来说，可以按照以下步骤：

1. 统计字符频率：首先统计每个字母在字符串中出现的次数。
2. 选择最小字母：在遍历字符串时，选择当前可以选的最小字母，同时确保后面还有足够的该字母可以补上。

以 "bcabc" 为例：

统计最后出现位置：{'b':3, 'c':4, 'a':2}。

遍历：

1. 'b'：栈为空，压入 'b'，seen={'b'}。
2. 'c'：'c' > 'b'，压入 'c'，seen={'b','c'}。
3. 'a'：'a' < 'c'，且 'c' 的最后位置是 4 > 2，弹出 'c'；'a' < 'b'，且 'b' 的最后位置是 3 > 2，弹出 'b'；压入 'a'，seen={'a'}。
4. 'b'：'b' > 'a'，压入 'b'，seen={'a','b'}。
5. 'c'：'c' > 'b'，压入 'c'，seen={'a','b','c'}。

结果：'a' + 'b' + 'c' = "abc"。

```c++
class Solution {
public:
    string removeDuplicateLetters(string s) {
        stack<char> st;
        vector<int> last_occurrence(26, -1);
        vector<bool> seen(26, false);

        for (int i = 0; i < s.size(); ++i) {
            last_occurrence[s[i] - 'a'] = i;
        }

        for (int i = 0; i < s.size(); ++i) {
            char c = s[i];
            int pos = c - 'a';
            if (seen[pos]) continue;

            while (!st.empty() && c < st.top() && i < last_occurrence[st.top() - 'a']) {
                seen[st.top() - 'a'] = false;
                st.pop();
            }

            st.push(c);
            seen[pos] = true;
        }

        string result;
        while (!st.empty()) {
            result += st.top();
            st.pop();
        }
        reverse(result.begin(), result.end());
        return result;
    }
};
```
