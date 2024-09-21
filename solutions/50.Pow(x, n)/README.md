# 50. Pow(x, n)

### 题目描述

实现pow(x,n)这个函数，即计算x的n次方。

### 题目解析

若x等于0,直接返回。
若n小于0,将其转换为正数，保留标记指示将最终的值反转。

### 代码实现

###### c++

```c++
class Solution {
public:
    double myPow(double x, int n) {
        return binaryExp(x, static_cast<long>(n));
    }

private:
    double binaryExp(double x, long n) {
        if (n == 0) return 1;
        if (n < 0) return 1.0 / binaryExp(x, -n);
       
        return n % 2 == 1 ? x * binaryExp(x * x, (n - 1) / 2) : binaryExp(x * x, n / 2);
    }
};
```