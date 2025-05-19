# 342. Power of Four

### Description

Given an integer n, return true if it is a power of four. Otherwise, return false.

An integer n is a power of four, if there exists an integer x such that n == 4x.

### Example 

###### Example I

```
Input: n = 16
Output: true
```

###### Example II

```
Input: n = 5
Output: false
```

###### Example III

```
Input: n = 1
Output: true
```

### Solution

可以先采用与Power of Three哪一题一样的方式完成：

```c++
class Solution {
public:
    bool isPowerOfFour(int n) {
        while (n > 0 && n % 4 == 0) n /= 4;
        return n == 1;
    }
};
```

随后我们考虑不需要循环的方式，我们知道是4的幂首先一定是2的幂，且1所处的位置一定是奇数位（或者模3一定为1）。

```c++
class Solution {
public:
    bool isPowerOfFour(int n) {
        return n > 0 && !(n & (n - 1)) && n % 3 == 1;
    }
};
```

或

```c++
class Solution {
public:
    bool isPowerOfFour(int n) {
        return n > 0 && !(n & (n - 1)) && (n & 0x55555555);
    }
};
```