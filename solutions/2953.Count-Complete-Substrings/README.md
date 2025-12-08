# 2953. Count Complete Substrings

**Tags:** Sliding Window

### Description

You are given a string word and an integer k.

A substring s of word is complete if:

- Each character in s occurs exactly k times.
- The difference between two adjacent characters is at most 2. That is, for any two adjacent characters c1 and c2 in s, the absolute difference in their positions in the alphabet is at most 2.
Return the number of complete substrings of word.

A substring is a non-empty contiguous sequence of characters in a string.

### Example

###### Example I

> Input: word = "igigee", k = 2
> Output: 3
> Explanation: The complete substrings where each character appears exactly twice and the difference between adjacent characters is at most 2 are: igigee, igigee, igigee.

###### Example II

> Input: word = "aaabbbccc", k = 3
> Output: 6
> Explanation: The complete substrings where each character appears exactly three times and the difference between adjacent characters is at most 2 are: aaabbbccc, aaabbbccc, aaabbbccc, aaabbbccc, aaabbbccc, aaabbbccc.

### Solution

条件二针对的是连续的字符串，所以我们可以从原字符串中按照此规则摘取出合适的子字符串，再检查是否满足条件一。

条件一就麻烦一点，我们考虑可能出现的字符种类数，暴力检查。题目要求每个字符一定要出现 K 次，M种不同字符得到的字符串长度就是 KM。

在字符串中维持一个这样的滑动窗口，检查每一个字符是否确实满足条件。

```c++
class Solution {
public:
    int countCompleteSubstrings(string word, int k) {
        int n = word.size(), ans = 0;
        for (int i = 0; i < n; ) {
            int st = i;
            for (i++; i < n && abs(word[i] - word[i - 1]) <= 2; i++);
            ans += f(word.substr(st, i - st), k);
        }
        return ans;
    }

private:
    int f(string s, int k) {
        int res = 0;
        for (int m = 1; m <= 26; m++) {
            int cnt[26]{};
            auto check = [&]() {
                for (int i = 0; i < 26; i++) {
                    if (cnt[i] && cnt[i] != k) return;
                }
                res++;
            };

            for (int r = 0; r < s.size(); r++) {
                cnt[s[r] - 'a']++;
                int l = r + 1 - k * m;
                if (l >= 0) {
                    check();
                    cnt[s[l] - 'a']--;
                }
            }
        }
        return res;
    }
};
```
