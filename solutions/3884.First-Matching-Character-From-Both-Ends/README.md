# 3884. First Matching Character From Both Ends

**Tags:** String

### Description

You are given a string s of length n consisting of lowercase English letters.

Return the smallest index i such that s[i] == s[n - i - 1].

If no such index exists, return -1.

### Example

###### Example I

> Input: s = "abcacbd"
> Output: 1
> Explanation:
> At index i = 1, s[1] and s[5] are both 'b'.
> No smaller index satisfies the condition, so the answer is 1.

###### Example II

> Input: s = "abc"
> Output: 1
> Explanation:
> ​​​​​​At index i = 1, the two compared positions coincide, so both characters are 'b'.
> No smaller index satisfies the condition, so the answer is 1.

###### Example III

> Input: s = "abcdab"
> Output: -1
> Explanation:
> ​​​​​​​For every index i, the characters at positions i and n - i - 1 are different.
> Therefore, no valid index exists, so the answer is -1.

### Solution

按要求实现即可

```c++
class Solution {
public:
    int firstMatchingIndex(string s) {
        int n = s.size();
        for (int i = 0; i < n; i++) {
            if (n - i - 1 < 0) break;
            if (s[i] == s[n - i - 1]) return i;
        }
        return -1;
    }
};
```