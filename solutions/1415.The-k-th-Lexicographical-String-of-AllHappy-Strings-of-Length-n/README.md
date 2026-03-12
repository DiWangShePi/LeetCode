# 1415. The k-th Lexicographical String of All Happy Strings of Length n

**Tags:** Math

### Description

A happy string is a string that:

- consists only of letters of the set ['a', 'b', 'c'].
- s[i] != s[i + 1] for all values of i from 1 to s.length - 1 (string is 1-indexed).
For example, strings "abc", "ac", "b" and "abcbabcbcb" are all happy strings and strings "aa", "baa" and "ababbc" are not happy strings.

Given two integers n and k, consider a list of all happy strings of length n sorted in lexicographical order.

Return the kth string of this list or return an empty string if there are less than k happy strings of length n.

### Example

###### Example I

> Input: n = 1, k = 3
> Output: "c"
> Explanation: The list ["a", "b", "c"] contains all happy strings of length 1. The third string is "c".

###### Example II

> Input: n = 1, k = 4
> Output: ""
> Explanation: There are only 3 happy strings of length 1.

###### Example III

> Input: n = 3, k = 9
> Output: "cab"
> Explanation: There are 12 different happy string of length 3 ["aba", "abc", "aca", "acb", "bab", "bac", "bca", "bcb", "cab", "cac", "cba", "cbc"]. You will find the 9th string = "cab"

### Solution

由于 happy string 要求相邻字符不一样，因此长度为 n 的 happy string 总共有 3 * 2 ^ (n - 1) 种。如果 K 大于这个数字，那就直接返回空字符串。

这是因为，第一个位子上的字符有三种可能，后面的每个位置上有两种可能。以 N = 3 为例，当第一个字符是 a 时，后面的两个位子各有两种可能，那么以 a 开头的就有四个不同的字符串。b和c同理。题目要求以字典序排序，我们就知道 a 开头的是前 4 个，b 开头的是中间四个，c 开头的是结尾的四个。

由此，我们根据 k 的大小逐步划分区域，找到最终的答案。

```c++
class Solution {
public:
    string getHappyString(int n, int k) {
        int count = 3;
        if (n > 1) count *= pow(2, n - 1);
        if (k > count) return "";

        string res = "";
        vector<char> chs = {'a', 'b', 'c'};
        for (int i = 0; i < n; i++) {
            int count = 1 << (n - i - 1);
            for (char c : chs) {
                if (!res.empty() && res.back() == c) {
                    continue;
                }
                if (k <= count) {
                    res.push_back(c);
                    break;
                }
                k -= count;
            }
        }
        return res;
    }
};
```
