# 227. Basic Calculator II

### Description

Given a string s which represents an expression, evaluate this expression and return its value. 

The integer division should truncate toward zero.

You may assume that the given expression is always valid. All intermediate results will be in the range of [-231, 231 - 1].

Note: You are not allowed to use any built-in function which evaluates strings as mathematical expressions, such as eval().

### Solution

用栈将所有数字保留下来，最后求和。

- 若是'+'，则将下一个读到的数字直接放入栈中。
- 若是'-'，则将下一个数字变为负值放入栈中。
- 若是'*'，则将栈顶的数字提出来，与当前读到的数字相乘后放入栈中。
- 若是'/'，则将栈顶的数字提出来，与当前的数字相除放入栈中。

> 遇到一个新的操作符时进行如上判断并更新

```c++
class Solution {
public:
    int calculate(string s) {
        stack<int> stk;
        int num = 0;
        char sign = '+'; 
        int n = s.size();

        for (int i = 0; i < n; ++i) {
            char c = s[i];

            if (isdigit(c)) {
                num = num * 10 + (c - '0');
            }

            if ((!isdigit(c) && c != ' ') || i == n - 1) {
                if (sign == '+') {
                    stk.push(num);
                } else if (sign == '-') {
                    stk.push(-num);
                } else if (sign == '*') {
                    int top = stk.top(); stk.pop();
                    stk.push(top * num);
                } else if (sign == '/') {
                    int top = stk.top(); stk.pop();
                    stk.push(top / num);
                }
                sign = c;
                num = 0;
            }
        }

        int result = 0;
        while (!stk.empty()) {
            result += stk.top();
            stk.pop();
        }
        return result;
    }
};
```
