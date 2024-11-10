# 97. Interleaving String

### Question Description

Given strings s1, s2, and s3, find whether s3 is formed by an interleaving of s1 and s2.

An interleaving of two strings s and t is a configuration where s and t are divided into n and m 
substrings. respectively, such that:

- s = s1 + s2 + ... + sn
- t = t1 + t2 + ... + tm
- |n - m| <= 1
The interleaving is s1 + t1 + s2 + t2 + s3 + t3 + ... or t1 + s1 + t2 + s2 + t3 + s3 + ...
Note: a + b is the concatenation of strings a and b.

**Example: **

```
Input: s1 = "aabcc", s2 = "dbbca", s3 = "aadbbcbcac"
Output: true
Explanation: One way to obtain s3 is:
Split s1 into s1 = "aa" + "bc" + "c", and s2 into s2 = "dbbc" + "a".
Interleaving the two splits, we get "aa" + "dbbc" + "bc" + "a" + "c" = "aadbbcbcac".
Since s3 can be obtained by interleaving s1 and s2, we return true.
```

```
Input: s1 = "aabcc", s2 = "dbbca", s3 = "aadbbbaccc"
Output: false
Explanation: Notice how it is impossible to interleave s2 with any other string to obtain s3.
```

```
Input: s1 = "", s2 = "", s3 = ""
Output: true
```

### Solution

如果s1和s2确实能组成s3，那么遍历s3时，当前字符要么属于s2，要么属于s1。

特别的，如果上一个字符属于s1，下一个字符属于s2，这代表进行了一次变化，即匹配的字符段从s1的变到s2了。
这一情况叠加可能出现的重复字符，我们做这一定义：若上一次匹配了s1中的元素，则这一次优先考虑s1是否成立。

但这样解法就会出问题，如s1="aa",s2="ab",s3="aaba"。判断为false，但理应为true。一个直接的补丁是在相同的时候考虑两种情况，这样就太麻烦了。

因此，我们转而考虑动态规划。定义f(i,j)为s1的前i个元素和s2的前j个元素是否能构成s3的前i+j个元素，我们发现当前值是否为true取决于以下两个条件：
1. s1的第i个元素等于s3的第i+j个元素，且s1的前i-1个元素可以和s2的前j个元素构成s3的前i-1+j个元素。
2. s2的第j个元素等于s3的第i+j个元素，且s2的前j-2个元素可以和s1的前i个元素构成s3的前i+j-1个元素。
这两个条件成立其一即可。

### Code Implemption

###### c++

```c++
class Solution {
public:
    bool isInterleave(string s1, string s2, string s3) {
        int m = s1.length(), n = s2.length(), l = s3.length();
        if (m + n != l) return false;

        vector<vector<bool>> board(m + 1, vector<bool>(n + 1, false));
        board[0][0] = true;
        for (int i = 0; i <= m; i++) {
            for (int j = 0; j <= n; j++) {
                int p = i + j - 1;
                if (i > 0) {
                    board[i][j] = board[i][j] || (board[i-1][j] && s1[i-1] == s3[p]);
                }
                if (j > 0) {
                    board[i][j] = board[i][j] || (board[i][j-1] && s2[j-1] == s3[p]);
                }
            }
        }
        return board[m][n];
    }
};
```