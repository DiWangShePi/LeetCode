# 224. Basic Calculator

### Description

Given a string s representing a valid expression, implement a basic calculator to evaluate it, and return the result of the evaluation.

Note: You are not allowed to use any built-in function which evaluates strings as mathematical expressions, such as eval().

### Solution

按要求实现即可

```c++
class Solution {
public:
    int calculate(string s) {
        int n = s.length();
        stack<int> stk;
        int result = 0;
        int sign = 1;
        int num = 0;
        bool hasNum = false;

        for (int i = 0; i < n; ++i) {
            char c = s[i];

            if (isdigit(c)) {
                num = num * 10 + (c - '0');
                hasNum = true;
            } else if (c == '+' || c == '-') {
                if (hasNum) {
                    result += sign * num;
                    num = 0;
                    hasNum = false;
                }

                if (c == '-') {
                    if (i == 0 || s[i - 1] == '(') {
                        sign = -1;
                        continue;
                    }
                }
                sign = (c == '+') ? 1 : -1;
            } else if (c == '(') {
                stk.push(result);
                stk.push(sign);
                result = 0;
                sign = 1;
            } else if (c == ')') {
                if (hasNum) {
                    result += sign * num;
                    num = 0;
                    hasNum = false;
                }
                int prevSign = stk.top(); stk.pop();
                int prevResult = stk.top(); stk.pop();
                result = prevResult + prevSign * result;
            }
        }

        if (hasNum) {
            result += sign * num;
        }

        return result;
    }
};
```
