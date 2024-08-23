# 22. Generate Parentheses

### 题目描述

给定`n`对括号，编写一个函数来生成所有格式正确的括号组合。

**示例：**

```
Input: n = 3
Output: ["((()))","(()())","(())()","()(())","()()()"]

Input: n = 1
Output: ["()"]
```

### 题目解析

使用DFS，可以生成$2^n$个字符串，再逐个排查，将合法的纳入到答案中。由于1<=n<=8，开销不会很大。

但为了保证字符串的合法，实际上字符串中一定会存在n个左括号和n个右括号。因此我们可以在字符串生成中
加入对已使用的左括号个数的指示，这样可以减少生成的字符串个数。

在这一思维上更进一步，实际上右括号只能在已存在左括号的时候出现。所以我们再加入一个值指示右括号的个数。
对树进行深度优先搜索，当左括号个数等于右括号个数等于n时结束遍历。

### 代码实现

###### c++

```c++
class Solution {
public:
    vector<string> generateParenthesis(int n) {
        vector<string> answer;
        oneParenthesis(0, 0, n, "", answer);
        return answer;
    }

private:
    void oneParenthesis(int left, int right, int n, string current, vector<string>& answer) {
        if (left == n && right == n) {
            answer.push_back(current);
            return;
        }

        if (left < n) {
            oneParenthesis(left+1, right, n, current+"(", answer);
        }

        if (right < left) {
            oneParenthesis(left, right+1, n, current+")", answer);
        }
    }
};
```