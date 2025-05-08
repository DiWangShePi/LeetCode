# 318. Maximum Product of Word Lengths

### Description

Given a string array words, return the maximum value of length(word[i]) * length(word[j]) where the two words do not share common letters. If no such two words exist, return 0.

### Example 

###### Example I

```
Input: words = ["abcw","baz","foo","bar","xtfn","abcdef"]
Output: 16
Explanation: The two words can be "abcw", "xtfn".
```

###### Example II

```
Input: words = ["a","ab","abc","d","cd","bcd","abcd"]
Output: 4
Explanation: The two words can be "ab", "cd".
```

###### Example III

```
Input: words = ["a","aa","aaa","aaaa"]
Output: 0
Explanation: No such pair of words.
```

### Solution

遍历考虑每两个字符串之间是否可以相乘，如果可以，是否更新了最大值。问题在于如何快速的判断两个字符串是否存在相同字符。

这一点可以采取位掩码的形式：
- 字符串1 = "abc" → 掩码: 000...111 (二进制)
- 字符串2 = "def" → 掩码: 000...111000 (二进制)
- 字符串3 = "cde" → 掩码: 000...111100 (二进制)
- "abc" & "def" = 0 → 无共同字符
- "abc" & "cde" = 000...100 ≠ 0 → 有共同字符'c'

```c++
class Solution {
public:
    int maxProduct(vector<string>& words) {
        int n = words.size();

        vector<int> mask(n, 0);
        vector<int> len(n, 0);
        for (int i = 0; i < n; i++) {
            string word = words[i];
            int length = word.size();

            for (int j = 0; j < length; j++) {
                mask[i] |= 1 << (word[j] - 'a');
            }
            len[i] = length;
        }

        int result = 0;
        for (int i = 0; i < n; i++) {
            for (int j = i + 1; j < n; j++) {
                if ((mask[i] & mask[j]) == 0) result = max(result, len[i] * len[j]);
            }
        }
        return result;
    }
};
```
