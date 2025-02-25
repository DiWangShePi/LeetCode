# 204. Count Primes

### Description

Given an integer n, return the number of prime numbers that are strictly less than n.

### Solution

Sieve of Eratosthenes

> 参考：https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes

### Implementation

###### c++

```c++
class Solution {
public:
    int countPrimes(int n) {
        int sqrt_n = sqrt(n);
        vector<bool> mark(n+1, true);
        for (int i = 2; i <= sqrt_n; i++) {
            if (mark[i]) {
                for (int j = i * i; j < n; j = j + i) {
                    mark[j] = false;
                }
            }
        }

        vector<bool> result;
        for (int i = 2; i < n; i++) {
            if (mark[i]) result.push_back(i);
        }
        return result.size();
    }
};
```

> 虽然但是，记答案还是最快的
