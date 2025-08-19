# 451. Sort Characters By Frequency

**Tags:** string

### Description

Given a string s, sort it in decreasing order based on the frequency of the characters. The frequency of a character is the number of times it appears in the string.

Return the sorted string. If there are multiple answers, return any of them.

### Example 

###### Example I

> Input: s = "tree"
> Output: "eert"
> Explanation: 'e' appears twice while 'r' and 't' both appear once.
> So 'e' must appear before both 'r' and 't'. Therefore "eetr" is also a valid answer.

###### Example II

> Input: s = "cccaaa"
> Output: "aaaccc"
> Explanation: Both 'c' and 'a' appear three times, so both "cccaaa" and "aaaccc" are valid answers.
> Note that "cacaca" is incorrect, as the same characters must be together.

###### Example III

> Input: s = "Aabb"
> Output: "bbAa"
> Explanation: "bbaA" is also a valid answer, but "Aabb" is incorrect.
> Note that 'A' and 'a' are treated as two different characters.

### Solution

遍历字符串，统计字符出现次数，再按照要求重组。

```c++
class Solution {
public:
    string frequencySort(string s) {
        vector<vector<int>> fre(76, vector<int>(2, 0));
        for (int i = 0; i < fre.size(); i++) fre[i][0] = i;
        for (char c : s) fre[c - '0'][1]++;

        sort(fre.begin(), fre.end(), [](const vector<int>& a, const vector<int>& b) {
            return a[1] > b[1];
        });

        string an;
        for (int i = 0; i < fre.size(); i++) {
            if (fre[i][1] != 0) {
                string t = string(fre[i][1], '0' + fre[i][0]);
                an += t;
            }
        }
        return an;
    }
};
```
