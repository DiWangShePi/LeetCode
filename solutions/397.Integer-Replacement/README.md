# 397. Integer Replacement

### Description

Given a positive integer n, you can apply one of the following operations:

1. If n is even, replace n with n / 2.
2. If n is odd, replace n with either n + 1 or n - 1.
Return the minimum number of operations needed for n to become 1.

### Example 

###### Example I

> Input: n = 8
> Output: 3
> Explanation: 8 -> 4 -> 2 -> 1

###### Example II

> Input: n = 7
> Output: 4
> Explanation: 7 -> 8 -> 4 -> 2 -> 1
> or 7 -> 6 -> 3 -> 2 -> 1

###### Example III

> Input: n = 4
> Output: 2

### Solution

递归

```c++
class Solution {
public:
    int integerReplacement(int n) {
        return helper(n);
    }

private:
    int helper(long n) {
        if (n == 1) return 0;

        if (n % 2 == 0) return helper(n / 2) + 1;
        else return min(helper(n + 1), helper(n - 1)) + 1;
    }
};
```
