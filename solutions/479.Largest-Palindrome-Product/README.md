# 479. Largest Palindrome Product

### Description

Given an integer n, return the largest palindromic integer that can be represented as the product of two n-digits integers. Since the answer can be very large, return it modulo 1337.

### Example 

###### Example I

> Input: n = 2
> Output: 987
> Explanation: 99 x 91 = 9009, 9009 % 1337 = 987

###### Example II

> Input: n = 1
> Output: 9

### Solution

我一开始的想法是暴力枚举

```c++
class Solution {
public:
    int largestPalindrome(int n) {
        long base = pow(10, n) - 1;
        long long upper = base * base;

        for (long long i = upper; i > 0; i--) {
            if (check_pali(i) && check_prod(i, base)) return i % 1337;
        }
        return n;
    }

private:
    bool check_pali(long long n) {
        string t = to_string(n);
        int i = 0, j = t.size() - 1;
        while (i < j) {
            if (t[i] == t[j]) {
                i++;
                j--;
            } else break;
        }
        return i >= j;
    }

    bool check_prod(long long n, long long b) {
        long long base = sqrt(n);
        for (int i = base; i > 0; i--) {
            if (n % i == 0 && n / i <= b) return true;
        }
        return false;
    }
};
```

超时之后去看官方的解法是什么，发现也是枚举

```c++
class Solution {
public:
    int largestPalindrome(int n) {
        if (n == 1) return 9;

        int upper = pow(10, n) - 1;
        for (int left = upper; ; left--) {
            long p = left;
            for (int i = left; i > 0; i /= 10) {
                p = p * 10 + i % 10;
            }

            for (long i = upper; i * i >= p; i--) {
                if (p % i == 0) return p % 1337;
            }
        }
    }
};
```

评论区给出了真正的枚举解法

```c++
class Solution {
public:
    int largestPalindrome(int n) {
        vector<int> dict{9,987,123,597,677,1218,877,475};
        return dict[n - 1];
    }
};
```
