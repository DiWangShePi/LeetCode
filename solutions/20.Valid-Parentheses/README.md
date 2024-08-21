# 20. Valid Parentheses

### 题目描述

给定一个仅包含`(`、`)`、`{`、`}`、`[`、`]`的字符串，判断该字符串是否符合以下要求：

1. 左括号必须用相同类型的括号闭合。
2. 左括号必须按正确的顺序闭合。
3. 每个右括号都有与之对应的相同类型的左括号。

### 题目解析

维护一个数组，当遇到某一类型的左括号时，将其加入到数组中。
当遇到某一类型的右括号时，检查数组最末尾的元素，是否为与之对应的左括号。
若是，继续到下一个；若不是，则不匹配，可直接退出。

### 代码实现

```c++
#include <string>
#include <vector>

class Solution {
public:
    bool isValid(std::string s) {
        std::vector<char> stack;
        
        for (char currentChar : s) {
            if (currentChar == '(' || currentChar == '[' || currentChar == '{') {
                stack.push_back(currentChar);
            } else {
                if (stack.empty()) return false;
                char topChar = stack.back();
                if ((currentChar == ')' && topChar != '(') ||
                    (currentChar == ']' && topChar != '[') ||
                    (currentChar == '}' && topChar != '{')) {
                    return false;
                }
                stack.pop_back();
            }
        }
        
        return stack.empty();
    }
};
```