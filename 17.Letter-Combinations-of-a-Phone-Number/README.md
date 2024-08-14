# 17. Letter Combinations of a Phone Number

### 题目描述

给定一个包含数字2-9的字符串，返回该数字可能表示的所有可能的字母组合。以任意顺序返回答案。
下面给出了数字到字母的映射。注意，1不能映射到任何字母。

![](./1200px-telephone-keypad2svg.png)

### 题目解析

采用回溯算法

### 代码实现

###### c++

```c++
#include <vector>
#include <string>

using namespace std;

class Solution {
public:
    vector<string> letterCombinations(string digits) {
        if (digits.empty()) {
            return {};
        }

        unordered_map<char, string> dict = {
            {'2', "abc"}, {'3', "def"}, {'4', "ghi"},
            {'5', "jkl"}, {'6', "mno"}, {'7', "pqrs"},
            {'8', "tuv"}, {'9', "wxyz"}
        };

        vector<string> combinations;
        backtrack(0, "", digits, dict, combinations);
        return combinations;
    }

private:
    void backtrack(int index, string path, const string& digits, const unordered_map<char, string>& letters, vector<string>& combinations) {
        if (path.size() == digits.size()) {
            combinations.push_back(path);
            return;
        }

        string possibleLetters = letters.at(digits[index]);
        for (char letter : possibleLetters) {
            backtrack(index + 1, path + letter, digits, letters, combinations);
        }
    }
};
```
