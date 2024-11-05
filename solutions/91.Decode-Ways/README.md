# 91. Decode Ways

### Question Description

You have intercepted a secret message encoded as a string of numbers. The message is decoded via the following mapping:

"1" -> 'A'

"2" -> 'B'

...

"25" -> 'Y'

"26" -> 'Z'

However, while decoding the message, you realize that there are many different ways you can decode the message because some codes are contained in other codes ("2" and "5" vs "25").

For example, "11106" can be decoded into:

"AAJF" with the grouping (1, 1, 10, 6)
"KJF" with the grouping (11, 10, 6)
The grouping (1, 11, 06) is invalid because "06" is not a valid code (only "6" is valid).
Note: there may be strings that are impossible to decode.

Given a string s containing only digits, return the number of ways to decode it. If the entire string cannot be decoded in any valid way, return 0.

The test cases are generated so that the answer fits in a 32-bit integer.

```
Input: s = "12"

Output: 2

Explanation:

"12" could be decoded as "AB" (1 2) or "L" (12).
```

```
Input: s = "226"

Output: 3

Explanation:

"226" could be decoded as "BZ" (2 26), "VF" (22 6), or "BBF" (2 2 6).
```

```
Input: s = "06"

Output: 0

Explanation:

"06" cannot be mapped to "F" because of the leading zero ("6" is different from "06"). In this case, the string is not a valid encoding, so return 0.
```

### Solution

状态转移矩阵，每次考虑走一步或者两步。

### Code Implemption

###### c++

```c++
class Solution {
public:
    int numDecodings(string s) {
        if (s.empty() || s[0] == '0') return 0;
        
        int n = s.length();
        vector<int> dp(n + 1, 0);
        dp[n] = 1;  

        for (int i = n - 1; i >= 0; --i) {
            if (s[i] != '0') {  
                dp[i] += dp[i + 1];  
                if (i + 1 < n && ((s[i] == '1') || (s[i] == '2' && s[i + 1] <= '6'))) {
                    dp[i] += dp[i + 2]; 
                }
            }
        }

        return dp[0];  
    }
};
```

dfs using cashed memory

```c++
class Solution {
public:
    int numDecodings(string s) {
        if (s.empty() || s[0] == '0') return 0;
        
        unordered_map<int, int> memo;
        return dfs(s, 0, memo);
    }

    int dfs(const string& s, int index, unordered_map<int, int>& memo) {
        if (index == s.length()) return 1;
        if (s[index] == '0') return 0;
        
        if (memo.find(index) != memo.end()) {
            return memo[index];  // Use cached result
        }
        
        int result = dfs(s, index + 1, memo);
        
        if (index + 1 < s.length()) {
            if ((s[index] == '1') || (s[index] == '2' && s[index + 1] <= '6')) {
                result += dfs(s, index + 2, memo);
            }
        }
        
        memo[index] = result;
        return result;
    }
};
```