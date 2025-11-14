# 1456. Maximum Number of Vowels in a Substring of Given Length

**Tags:** Sliding Window

### Description

Given a string s and an integer k, return the maximum number of vowel letters in any substring of s with length k.

Vowel letters in English are 'a', 'e', 'i', 'o', and 'u'.

### Example

###### Example I

> Input: s = "abciiidef", k = 3
> Output: 3
> Explanation: The substring "iii" contains 3 vowel letters.

###### Example II

> Input: s = "aeiou", k = 2
> Output: 2
> Explanation: Any substring of length 2 contains 2 vowels.

###### Example III

> Input: s = "leetcode", k = 3
> Output: 2
> Explanation: "lee", "eet" and "ode" contain 2 vowels.

### Solution

滑动窗口，参见：https://leetcode.cn/discuss/post/3578981/ti-dan-hua-dong-chuang-kou-ding-chang-bu-rzz7/

```c++
class Solution {
public:
    int maxVowels(string s, int k) {
        int index = 0;
        long an = INT_MIN, current = 0;
        while (index < s.size()) {
            if (isVowel(s[index])) current++;
            if (index >= k && isVowel(s[index - k])) current--; 

            an = max(an, current);
            index++;
        }
        return an;
    }

private:
    bool isVowel(char c) {
        return c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u';
    }
};
```
