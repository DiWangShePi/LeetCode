# 241. Different Ways to Add Parentheses

### Description

Given a string expression of numbers and operators, return all possible results from computing all the different possible ways to group numbers and operators. You may return the answer in any order.

The test cases are generated such that the output values fit in a 32-bit integer and the number of different results does not exceed 104.

### Solution

对于指定计算符号，将其作为分离点将表达式分为两部分，枚举两侧表达式的所有可能结果，再用当前计算符号加以组合。枚举所有可能的分割点，即为最终结果。

```c++
class Solution {
public:
    unordered_map<string, vector<int>> memo;

    vector<int> diffWaysToCompute(string expression) {
        if (memo.count(expression)) {
            return memo[expression];
        }

        vector<int> res;
        for (int i = 0; i < expression.size(); ++i) {
            char c = expression[i];
            if (c == '+' || c == '-' || c == '*') {
                string left = expression.substr(0, i);
                string right = expression.substr(i + 1);
                vector<int> leftResults = diffWaysToCompute(left);
                vector<int> rightResults = diffWaysToCompute(right);

                for (int l : leftResults) {
                    for (int r : rightResults) {
                        if (c == '+') res.push_back(l + r);
                        else if (c == '-') res.push_back(l - r);
                        else if (c == '*') res.push_back(l * r);
                    }
                }
            }
        }

        if (res.empty()) {
            res.push_back(stoi(expression));
        }

        memo[expression] = res;
        return res;
    }
};
```
