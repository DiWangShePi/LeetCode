# 263. Ugly Number

### Description

An ugly number is a positive integer which does not have a prime factor other than 2, 3, and 5.

Given an integer n, return true if n is an ugly number.

### Solution

把能除的2，3，5都除了，剩下的不是1就代表不是

```c++
class Solution {
public:
    bool isUgly(int n) {
        if (n < 1) return false;
        while (n % 2 == 0) n = n / 2;
        while (n % 3 == 0) n = n / 3;
        while (n % 5 == 0) n = n / 5;
        return n == 1;
    }
};
```
