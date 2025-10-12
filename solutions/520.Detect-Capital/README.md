# 520. Detect Capital

**Tags:** Math

### Description

We define the usage of capitals in a word to be right when one of the following cases holds:

- All letters in this word are capitals, like "USA".
- All letters in this word are not capitals, like "leetcode".
- Only the first letter in this word is capital, like "Google".
Given a string word, return true if the usage of capitals in it is right.

### Example

###### Example I

> Input: word = "USA"
> Output: true

###### Example II

> Input: word = "FlaG"
> Output: false

### Solution

根据第一位和第二位的大小写，判断接下来所有的字符应该是大写还是小写。

```c++
class Solution {
public:
    bool detectCapitalUse(string word) {
        if (word.size() == 1) return true;

        bool upperCase = false;
        if (word[0] - 'Z' <= 0 && word[1] - 'Z' <= 0) upperCase = true;
        if (word[0] - 'Z' > 0) upperCase = false;

        for (int i = 1; i < word.size(); i++) {
            if (!((word[i] - 'Z' <= 0) == upperCase)) return false;
        }
        return true;
    }
};
```
