# 10. Regular Expression Matching

### 题目描述

给你一个字符串 s 和一个字符规律 p，请你来实现一个支持 '.' 和 '*' 的正则表达式匹配。

'.' 匹配任意单个字符
'*' 匹配零个或多个前面的那一个元素
所谓匹配，是要涵盖 整个 字符串 s的，而不是部分字符串。

**示例：**

```
输入：s = "aa", p = "a"
输出：false
解释："a" 无法匹配 "aa" 整个字符串。
```

```
输入：s = "aa", p = "a*"
输出：true
解释：因为 '*' 代表可以匹配零个或多个前面的那一个元素, 在这里前面的元素就是 'a'。因此，字符串 "aa" 可被视为 'a' 重复了一次。
```

```
输入：s = "ab", p = ".*"
输出：true
解释：".*" 表示可匹配零个或多个（'*'）任意字符（'.'）。
```

### 题目解析

题目中的匹配是一个逐步匹配的过程：我们每次从字符串P中取出一个字符或字符+星号的组合，并在S中进行匹配。

- 对于P中的一个字符而言，它只能在S中普配一个字符，这个匹配有唯一性。
- 对于P中的字符+星号组合而言，它可以在S中匹配任意自然数个字符，并不具有唯一性。

我们用$f[i][j]$表示S的前$i$个字符与P的前$j$个字符是否能够匹配。在进行状态转移时，我们考虑P的第$j$个字符的匹配情况：

- 如果P的第$j$个字符是一个小写字母，那我们必须在S中匹配一个相同的小写字母，即：

$$
f[i][j] = 
\begin{cases}
  & f[i-1][j-1], \text{ if } s[i]=p[j] \\
  & false, \text{ if } s[i] \ne p[j]
\end{cases}
$$

也就是说，如果S的第$i$个字符与P的第$j$个字符不相同，那么无法进行匹配；否则我们可以匹配两个字符串的最后一个字符。完整的匹配结果则取决于两个字符串前面 的部分。

- 如果P的第$j$个字符是一个*，那我们必可以对P的第$j-1$个字符匹配任意自然数次。在匹配0次的情况下，我们有：

$$
f[i][j] = f[i][j-2]
$$

如果我们通过这种方法进行转移，那么我们就需要枚举这个组合到底匹配了 s 中的几个字符，会增导致时间复杂度增加，并且代码编写起来十分麻烦。我们不妨换个角度考虑这个问题：字母 + 星号的组合在匹配的过程中，本质上只会有两种情况：

1. 匹配 s 末尾的一个字符，将该字符扔掉，而该组合还可以继续进行匹配；

2. 不匹配字符，将该组合扔掉，不再进行匹配。

按照这个角度进行思考，我们可以写出很精巧的状态转移方程：

$$
f[i][j] = 
\begin{cases}
  & f[i-1][j] or f[i][j-2], \text{ if } s[i]=p[j] \\
  & f[i][j-2], \text{ if } s[i] \ne p[j]
\end{cases}
$$

### 代码实现

###### c++

```c++
#include <string>

using namespace std;

class Solution {
public:
    bool isMatch(string s, string p) {
        int m = s.size();
        int n = p.size();

        auto matches = [&](int i, int j) {
            if (i == 0) {
                return false;
            }
            if (p[j - 1] == '.') {
                return true;
            }
            return s[i - 1] == p[j - 1];
        };

        vector<vector<int>> f(m + 1, vector<int>(n + 1));
        f[0][0] = true;
        for (int i = 0; i <= m; ++i) {
            for (int j = 1; j <= n; ++j) {
                if (p[j - 1] == '*') {
                    f[i][j] |= f[i][j - 2];
                    if (matches(i, j - 1)) {
                        f[i][j] |= f[i - 1][j];
                    }
                }
                else {
                    if (matches(i, j)) {
                        f[i][j] |= f[i - 1][j - 1];
                    }
                }
            }
        }
        return f[m][n];
    }
};
```

###### rust

```rust
impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let (s, p) = (s.into_bytes(), p.into_bytes());
        let columns = p.len() + 1;
        let mut cache = vec![false; columns * (s.len() + 1)];
        let cache_len = cache.len();
        let last_row = &mut cache[cache_len - columns..];

        *last_row.last_mut().unwrap() = true;

        for j in (0..p.len()).rev() {
            if p.get(j + 1) == Some(&b'*') {
                last_row[j] = last_row[j + 2];
            }
        }

        for (i, s_i) in s.iter().enumerate().rev() {
            for (j, p_j) in p.iter().enumerate().rev() {
                cache[columns * i + j] = match (*p_j == b'.' || p_j == s_i, p.get(j + 1) == Some(&b'*')) {
                    (false, false) => continue,
                    (false, true) => cache[columns * i + (j + 2)],
                    (true, false) => cache[columns * (i + 1) + (j + 1)],
                    (true, true) => cache[columns * i + (j + 2)] || cache[columns * (i + 1) + j],
                }
            }
        }

        cache[0]
    }
}
```