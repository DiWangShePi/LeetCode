# 483. Smallest Good Base

### Description

Given an integer n represented as a string, return the smallest good base of n.

We call k >= 2 a good base of n, if all digits of n base k are 1's.

### Example

###### Example I

> Input: n = "13"
> Output: "3"
> Explanation: 13 base 3 is 111.

###### Example II

> Input: n = "4681"
> Output: "8"
> Explanation: 4681 base 8 is 11111.

###### Example III

> Input: n = "1000000000000000000"
> Output: "999999999999999999"
> Explanation: 1000000000000000000 base 999999999999999999 is 11.

### Solution

按照题目的要求，我们可以给出这个暴力解：

```c++
class Solution {
public:
    string smallestGoodBase(string n) {
        long long N = stoll(n);
        for (long long i = 2; i < N; i++) {
            if (check(N, i)) return to_string(i);
        }
        return n;
    }

private:
    bool check(long long n, long long base) {
        while (n != 0) {
            if (n % base != 1) return false;
            n /= base;
        }
        return true;
    }
};
```

但这个解连样例也过不了，我们需要的是数学

> 参考：https://leetcode.cn/problems/smallest-good-base/solutions/832832/zui-xiao-hao-jin-zhi-by-leetcode-solutio-csqn/

```c++
class Solution {
public:
    string smallestGoodBase(string n) {
        long nVal = stol(n);
        int mMax = floor(log(nVal) / log(2));
        for (int m = mMax; m > 1; m--) {
            int k = pow(nVal, 1.0 / m);
            long mul = 1, sum = 1;
            for (int i = 0; i < m; i++) {
                mul *= k;
                sum += mul;
            }
            if (sum == nVal) {
                return to_string(k);
            }
        }
        return to_string(nVal - 1);
    }
};
```
