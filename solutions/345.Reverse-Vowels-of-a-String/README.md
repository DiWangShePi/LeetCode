# 345. Reverse Vowels of a String

### Description

Given a string s, reverse only all the vowels in the string and return it.

The vowels are 'a', 'e', 'i', 'o', and 'u', and they can appear in both lower and upper cases, more than once.

### Example 

###### Example I

```
Input: s = "IceCreAm"
Output: "AceCreIm"

Explanation:
The vowels in s are ['I', 'e', 'e', 'A']. On reversing the vowels, s becomes "AceCreIm".
```

###### Example II

```
Input: s = "leetcode"
Output: "leotcede"
```

### Solution

两个指针指向开头和末尾，往中间遍历并将发现到的元音交换

```c++
class Solution {
public:
    string reverseVowels(string& s) {
        int i = 0, j = s.size() - 1;

        while (i < j) {
            while (i < j && !isVowel(s[i])) i++;
            while (i < j && !isVowel(s[j])) j--;
            if (i < j) {
                swap(s[i], s[j]);
                i++;
                j--;
            }
        }
        return s;
    }

private:
    const unordered_set<char> vowels{'a', 'e', 'i', 'o', 'u'};

    bool isVowel(char c) {
        return vowels.count(tolower(c));
    }
};
```

也可以遍历一遍将发现到的元音收集起来，然后倒着遍历一遍再填回去

```c++
class Solution {
public:
    std::string reverseVowels(std::string s) {
        std::vector<char> vowels;

        for (int i = static_cast<int>(s.size()) - 1; i >= 0; --i) {
            if (isVowel(s[i])) {
                vowels.push_back(s[i]);
            }
        }

        int k = 0;
        for (int i = 0; i < static_cast<int>(s.size()); ++i) {
            if (isVowel(s[i])) {
                s[i] = vowels[k++];
            }
        }
        return s;
    }

private:
    bool isVowel(char c) {
        c = std::tolower(static_cast<unsigned char>(c));
        return c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u';
    }
};
```
