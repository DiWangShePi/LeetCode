# 187. Repeated DNA Sequences

### Description

The DNA sequence is composed of a series of nucleotides abbreviated as 'A', 'C', 'G', and 'T'.

For example, "ACGAATTCCG" is a DNA sequence.
When studying DNA, it is useful to identify repeated sequences within the DNA.

Given a string s that represents a DNA sequence, return all the 10-letter-long sequences (substrings) that occur more than once in a DNA molecule. You may return the answer in any order.

### Solution

遍历字符串，每次截取长度为10的子字符串，记录在哈希表中。

当哈希表该项的值为2时，将其记录在最终答案中。

### Implementation

###### c++

```c++
class Solution {
public:
    vector<string> findRepeatedDnaSequences(string s) {
        if (s.size() < 10) return {};

        vector<string> an;
        unordered_map<string, int> dict;
        for (int i = 0; i <= s.size() - 10; i++) {
            string current = s.substr(i, 10);
            if (++dict[current] == 2) an.push_back(current);
        }
        return an;
    }
};
```
