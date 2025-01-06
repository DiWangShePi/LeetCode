# 151. Reverse Words in a String

### Description

Given an input string s, reverse the order of the words.

A word is defined as a sequence of non-space characters. The words in s will be separated by at least one space.

Return a string of the words in reverse order concatenated by a single space.

Note that s may contain leading or trailing spaces or multiple spaces between two words. The returned string should only have a single space separating the words. Do not include any extra spaces.

### Solution

依据空格分割成字符串列表，将列表倒过来重组即可

### Implementation

###### c++

```c++
class Solution {
public:
    string reverseWords(string s) {
        std::vector<std::string> result;
        std::istringstream stream(s);
        std::string word;

        while (stream >> word) { 
            result.push_back(word);
        }

        std::string an = "";
        for (int i = result.size() - 1; i > 0; i--) {
            an = an + result[i] + " ";
        }
        an = an + result[0];
        return an;
    }
};
```
