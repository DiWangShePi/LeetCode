# 509. Fibonacci Number

### Description

The Fibonacci numbers, commonly denoted F(n) form a sequence, called the Fibonacci sequence, such that each number is the sum of the two preceding ones, starting from 0 and 1. That is,

> F(0) = 0, F(1) = 1
> F(n) = F(n - 1) + F(n - 2), for n > 1.

Given n, calculate F(n).

### Example 

###### Example I

> Input: n = 2
> Output: 1
> Explanation: F(2) = F(1) + F(0) = 1 + 0 = 1.

###### Example II

> Input: n = 3
> Output: 2
> Explanation: F(3) = F(2) + F(1) = 1 + 1 = 2.

###### Example III

> Input: n = 4
> Output: 3
> Explanation: F(4) = F(3) + F(2) = 2 + 1 = 3.

### Solution

遍历一遍，按要求计算：

```c++
class Solution {
public:
    int fib(int n) {
        if (n == 0) return 0;
        if (n == 1) return 1;

        n -= 2;
        int f_0 = 0, f_1 = 1, t = 0;
        while (n > -1) {
            t = f_1;
            f_1 = f_1 + f_0;
            f_0 = t;
            n--;
        }
        return f_1;
    }
};
```

或者根据公式直接计算

```c++
class Solution {
public:
    int fib(int n) {
        double sqrt5 = sqrt(5);
        double fibN = pow((1 + sqrt5) / 2, n) - pow((1 - sqrt5) / 2, n);
        return round(fibN / sqrt5);
    }
};
```
