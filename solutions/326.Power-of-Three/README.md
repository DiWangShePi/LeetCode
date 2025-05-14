# 326. Power of Three

### Description

Given an integer n, return true if it is a power of three. Otherwise, return false.

An integer n is a power of three, if there exists an integer x such that n == 3^x.

### Example 

###### Example I

```
Input: n = 27
Output: true
Explanation: 27 = 3^3
```

###### Example II

```
Input: n = 0
Output: false
Explanation: There is no x where 3^x = 0.
```

###### Example III

```
Input: n = -1
Output: false
Explanation: There is no x where 3^x = (-1).
```

### Solution

解法其一，不断除

```c++
class Solution {
public:
    bool isPowerOfThree(int n) {
        while (n > 0 && n % 3 == 0) n /= 3;
        return n == 1;
    }
};
```

解法其二，根据给定的范围（即先验知识）进行简化。

> 在题目给定的 32 位有符号整数的范围内，最大的 3 的幂为 3^19=1162261467。我们只需要判断 n 是否是 3^19的约数即可。

```c++
class Solution {
public:
    bool isPowerOfThree(int n) {
        return n > 0 && 1162261467 % n == 0;
    }
};
```
