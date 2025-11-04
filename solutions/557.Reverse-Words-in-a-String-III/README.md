# 557. Reverse Words in a String III

### Description

Given a string s, reverse the order of characters in each word within a sentence while still preserving whitespace and initial word order.

### Example

###### Example I

> Input: s = "Let's take LeetCode contest"
> Output: "s'teL ekat edoCteeL tsetnoc"

###### Example II

> Input: s = "Mr Ding"
> Output: "rM gniD"

### Solution

按要求实现即可

```c++
class Solution {
public:
    string reverseWords(string s) {
        int index = 0;
        for (int i = 0; i < s.size(); i++) {
            if (s[i] != ' ') continue;

            reverse(s.begin() + index, s.begin() + i);
            index = i + 1;
        }
        if (index < s.size()) reverse(s.begin() + index, s.end());
        return s;
    }
};
```
