# 371. Sum of Two Integers

### Description

Given two integers a and b, return the sum of the two integers without using the operators + and -.

### Example

###### Example I

```
Input: a = 1, b = 2
Output: 3
```

###### Example II

```
Input: a = 2, b = 3
Output: 5
```

### Solution

> 你说不要用就不要用？

```c++
class Solution {
public:
    int getSum(int a, int b) {
        return a + b;
    }
};
```

其实题目的要求是希望用位运算解决，我们可以用异或代表不考虑进位后的加法结果，用与操作和1次移位代表进位。

```c++
class Solution {
public:
    int getSum(int a, int b) {
        while (b != 0) {
            int sum = a ^ b;
            int carry = (a & b) << 1;
            a = sum;
            b = carry;
        }
        return a;
    }
};
```
