# 214. Shortest Palindrome

### Description

You are given a string s. You can convert s to a palindrome by adding characters in front of it.

Return the shortest palindrome you can find by performing this transformation.

### Solution

将原字符串s分为s1和s2两部分，确保s1为可得到的最长回文字符串，s2反序即为需要添加的字符串。

因此原题被改为寻找最长的s1。

寻找回文子串是一个开销十分之大的步骤，我们需要一些算法来加快这个检查过程。

1. 我们可以将字符串取哈希值，两个字符串哈希值相等只会在两个字符串相等的情况下出现。因此，枚举s1，分别计算s1和s1反序的哈希值，两者相同该字符串即为回文串。

2. KMP算法，或者马拉车算法

### Implementation

###### c++

```c++
class Solution {
public:
    string shortestPalindrome(string s) {
        int n = s.size();
        int base = 122, mod = 1000000007;
        int left = 0, right = 0, mul = 1;
        int best = -1;
        for (int i = 0; i < n; ++i) {
            left = ((long long)left * base + s[i]) % mod;
            right = (right + (long long)mul * s[i]) % mod;
            mul = (long long)mul * base % mod;

            if (left == right) {
                best = i;
            }
        }
        string add = (best == n - 1 ? "" : s.substr(best + 1, n));
        reverse(add.begin(), add.end());
        return add + s;
    }
};
```
