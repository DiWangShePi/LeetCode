# 231. Power of Two

### Description

Given an integer n, return true if it is a power of two. Otherwise, return false.

An integer n is a power of two, if there exists an integer x such that n == 2^x.

### Solution

n如果是2的幂，则其二进制表达会是一个1后面跟一些0。将n与n-1进行与操作会使得整个数字变为0，以此判断。

> c++中 == 运算符的优先级高于 &

### Implementation

###### c++

```c++
class Solution {
public:
    bool isPowerOfTwo(int n) {
        return (n & (n - 1)) == 0;
    }
};
```
