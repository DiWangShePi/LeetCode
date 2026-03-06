# 552. Student Attendance Record II

### Description

An attendance record for a student can be represented as a string where each character signifies whether the student was absent, late, or present on that day. The record only contains the following three characters:

- 'A': Absent.
- 'L': Late.
- 'P': Present.
Any student is eligible for an attendance award if they meet both of the following criteria:

- The student was absent ('A') for strictly fewer than 2 days total.
- The student was never late ('L') for 3 or more consecutive days.
Given an integer n, return the number of possible attendance records of length n that make a student eligible for an attendance award. The answer may be very large, so return it modulo 10^9 + 7.

### Example

###### Example I

> Input: n = 2
> Output: 8
> Explanation: There are 8 records with length 2 that are eligible for an award:
> "PP", "AP", "PA", "LP", "PL", "AL", "LA", "LL"
> Only "AA" is not eligible because there are 2 absences (there need to be fewer than 2).

###### Example II

> Input: n = 1
> Output: 3

###### Example III

> Input: n = 10101
> Output: 183236316

### Solution

动态规划。定义dp[i][j][k]为字符串长度为i时表示的，共有j个A，结尾有k个L的出勤记录。

根据规则，我们有：
- 当新的出勤记录为 P 时，当前值为前一天所有 j 和所有 k 之和。
- 当新的出勤记录为 A 时，前一天的 A 值必须为 0，因此为所有 k 之和。
- 当新的出勤记录为 L 时，前一天的 L 值必须小于等于 2。

```c++
class Solution {
public:
    int checkRecord(int n) {
        int MOD = 1000000007;
        vector<vector<vector<int>>> dp(n + 1, vector<vector<int>>(2, vector<int>(3, 0)));
        dp[0][0][0] = 1;

        for (int i = 1; i <= n; i++) {
            // new record is P
            for (int j = 0; j < 2; j++) {
                for (int k = 0; k < 3; k++) {
                    dp[i][j][0] = (dp[i][j][0] + dp[i - 1][j][k]) % MOD;
                }
            }

            // new record is A
            for (int k = 0; k < 3; k++) {
                dp[i][1][0] = (dp[i][1][0] + dp[i - 1][0][k]) % MOD;
            }

            // new record is L
            for (int j = 0; j < 2; j++) {
                for (int k = 1; k < 3; k++) {
                    dp[i][j][k] = (dp[i][j][k] + dp[i - 1][j][k - 1]) % MOD;
                }
            }
        }

        int an = 0;
        for (int j = 0; j < 2; j++) {
            for (int k = 0; k < 3; k++) {
                an = (an + dp[n][j][k]) % MOD;
            }
        }
        return an;
    }
};
```
