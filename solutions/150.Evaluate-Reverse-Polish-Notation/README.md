# 150. Evaluate Reverse Polish Notation

### Description

You are given an array of strings tokens that represents an arithmetic expression in a Reverse Polish Notation.

Evaluate the expression. Return an integer that represents the value of the expression.

Note that:

- The valid operators are '+', '-', '*', and '/'.
- Each operand may be an integer or another expression.
- The division between two integers always truncates toward zero.
- There will not be any division by zero.
- The input represents a valid arithmetic expression in a reverse polish notation.
- The answer and all the intermediate calculations can be represented in a 32-bit integer.

### Solution

保存一个栈，如果遇到数字则将其压入栈中，是运算符则将最近的两个数字提取出来，进行该运算。

### Implementation

###### c++

```c++
class Solution {
public:
    int evalRPN(std::vector<std::string>& tokens) {
        std::stack<int> stack;
        for (const std::string& token : tokens) {
            if (token == "+" || token == "-" || token == "*" || token == "/") {
                int s1 = stack.top(); stack.pop();
                int s2 = stack.top(); stack.pop();
                if (token == "+") stack.push(s2 + s1);
                if (token == "-") stack.push(s2 - s1);
                if (token == "*") stack.push(s2 * s1);
                if (token == "/") stack.push(s2 / s1);
            } else {
                stack.push(std::stoi(token));
            }
        }
        return stack.top();
    }
};
```

> 奇妙的写法，好奇为什么会更快

```c++
class Solution {
public:
    int evalRPN(vector<string>& tokens) {
    stack<int> mystack;

    for (const string& c : tokens) {
        if (c == "/") {
            int val2 = mystack.top(); mystack.pop(); // Note the order of popping
            int val1 = mystack.top(); mystack.pop();
            mystack.push(val1 / val2);
        } else if (c == "*") {
            int val2 = mystack.top(); mystack.pop();
            int val1 = mystack.top(); mystack.pop();
            mystack.push(val1 * val2);
        } else if (c == "-") {
            int val2 = mystack.top(); mystack.pop();
            int val1 = mystack.top(); mystack.pop();
            mystack.push(val1 - val2);
        } else if (c == "+") {
            int val2 = mystack.top(); mystack.pop();
            int val1 = mystack.top(); mystack.pop();
            mystack.push(val1 + val2);
        } else {
            // Convert string to integer and push onto the stack
            mystack.push(stoi(c));
        }
    }

    return mystack.top();
    }
};
```