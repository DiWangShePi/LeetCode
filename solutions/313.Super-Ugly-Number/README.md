# 313. Super Ugly Number

### Description

A super ugly number is a positive integer whose prime factors are in the array primes.

Given an integer n and an array of integers primes, return the nth super ugly number.

The nth super ugly number is guaranteed to fit in a 32-bit signed integer.

### Example

###### Example I

```
Input: n = 12, primes = [2,7,13,19]
Output: 32
Explanation: [1,2,4,7,8,13,14,16,19,26,28,32] is the sequence of the first 12 super ugly numbers given primes = [2,7,13,19].
```

```
Input: n = 1, primes = [2,3,5]
Output: 1
Explanation: 1 has no prime factors, therefore all of its prime factors are in the array primes = [2,3,5].
```

### Solution

参考263、264题，同样采取动态规划即可

```c++
class Solution {
public:
    int nthSuperUglyNumber(int n, vector<int>& primes) {
        vector<int> dp(n + 1);
        int k = primes.size();
        vector<int> pointer(k, 1);
        dp[1] = 1;

        for (int i = 2; i <= n; i++) {
            long long min_val = INT_MAX;
            for (int j = 0; j < k; j++) {
                long long next_val = (long long) primes[j] * dp[pointer[j]];
                if (next_val < min_val) {
                    min_val = next_val;
                }
            }

            dp[i] = (int) min_val;
            for (int j = 0; j < k; j++) {
                if (min_val == (long long) dp[pointer[j]] * primes[j]) pointer[j]++;
            }
        }
        return dp[n];
    }
};
```
