# 507. Perfect Number

### Description

A perfect number is a positive integer that is equal to the sum of its positive divisors, excluding the number itself. A divisor of an integer x is an integer that can divide x evenly.

Given an integer n, return true if n is a perfect number, otherwise return false.

### Example 

###### Example I

> Input: num = 28
> Output: true
> Explanation: 28 = 1 + 2 + 4 + 7 + 14
> 1, 2, 4, 7, and 14 are all divisors of 28.

###### Example II

> Input: num = 7
> Output: false

### Solution

遍历一遍，累加计算

```c++
class Solution {
public:
    bool checkPerfectNumber(int num) {
        if (num == 1) return false;

        int start = sqrt(num);
        int an = 0;

        while (start != 0) {
            if (num % start == 0) {
                an += num / start;
                an += start;
            }
            start--;
        }
        return an == 2 * num;
    }
};
```
