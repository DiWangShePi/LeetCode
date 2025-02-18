# 191. Number of 1 Bits

### Description

Given a positive integer n, write a function that returns the number of set bits in its binary representation (also known as the Hamming weight).

### Solution

判断n模2的余数是否为偶数，如果不是就加一。

每一次将n除以2。

> 仿照将数字转换为二进制的过程，并判断在这个过程中遇到了几个1.

### Implementation

###### c++

```c++
class Solution {
public:
    int hammingWeight(int n) {
        int result = 0;
        while (n != 0) {
            int res = n % 2;
            if (res % 2 != 0) result++;
            n = n / 2;
        }
        return result;
    }
};
```
