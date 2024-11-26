# 115. Distinct Subsequences

### Description

Given two strings s and t, return the number of distinct subsequences of s which equals t.

The test cases are generated so that the answer fits on a 32-bit signed integer.

### Solution

考虑深度优先搜索，每一步取或者不取当前字符，最后考虑获得的字符串是否和目标相等。
但这样时间太长了。

更合适的方式是动态规划。

### Implementation

###### c++

```c++
class Solution {
public:
    int numDistinct(string s, string t) {
        int m = s.length(), n = t.length();
        const int MOD = 1e9 + 7;  // 取模的常数
        
        // 使用 long long 以避免溢出
        vector<vector<long long>> dp(m + 1, vector<long long>(n + 1, 0));
        
        // 初始条件：空字符串t可以从任何位置的s中形成1种子序列
        for (int i = 0; i <= m; i++) {
            dp[i][0] = 1;
        }

        // 填充dp表
        for (int i = 1; i <= m; i++) {
            for (int j = 1; j <= n; j++) {
                // 如果当前字符相等，可以选择包括当前字符或者跳过
                if (s[i - 1] == t[j - 1]) {
                    dp[i][j] = (dp[i - 1][j - 1] + dp[i - 1][j]) % MOD;
                } else {
                    dp[i][j] = dp[i - 1][j] % MOD;
                }
            }
        }

        // 返回dp[m][n]，即s的前m个字符可以形成t的前n个字符的不同子序列个数
        return dp[m][n];
    }
};
```

a more elegant solution: 

```c++
class Solution {
public:
    int numDistinct(string s, string t) {
        vector<long long> track(t.length(), 0);
        int i, start_i;

        // 遍历字符串 s
        for(int j{0}; j < s.length(); j++){
            char c = s[j];  // 当前遍历到的字符是 s[j]
            
            i = start_i = min(j, static_cast<int>(t.length() - 1)); 
            // i 从 j 开始，初始值是 min(j, t.length() - 1)，即最多到 t 的最后一个字符

            // 内层循环处理 track 数组
            for(; start_i - i < static_cast<int>(s.length()) - static_cast<int>(t.length()) + 1 && i >= 1; i--){
                if(c == t[i])
                    track[i] += track[i-1];  // 如果 s[j] 等于 t[i]，将 track[i-1] 的结果累加到 track[i]，表示新增了一个子序列
            }

            if(c == t[0])
                track[0]++;  // 如果 s[j] 等于 t[0]，增加 track[0]，表示新增了一个匹配 t[0] 的子序列
        }

        return track[t.length()-1];  // 返回 track[t.length()-1]，即匹配到 t 的最后一个字符的子序列数量
    }
};
```