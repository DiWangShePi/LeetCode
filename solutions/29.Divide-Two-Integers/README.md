# 29. Divide Two Integers

### 题目描述

给定两个整数数字，被除数和除数，在不使用乘法，除法和模操作的前提下进行除法。

得到的结果应该往0规整，即大于0的时候向下取整，小于0的情况下向上取整。

假设我们处理的环境只能存储整数范围的数字。
如果得到的结果严格小于integer的下界，则返回下界。
如果得到的结果严格大于integer的上届，则返回上界。

**示例：**

```
Input: dividend = 10, divisor = 3
Output: 3
Explanation: 10/3 = 3.33333.. which is truncated to 3.
```

```
Input: dividend = 7, divisor = -3
Output: -2
Explanation: 7/-3 = -2.33333.. which is truncated to -2.
```

### 题目解析

若给定的两数字符号相同，则结果应为正数。反之则为负数。将符号记录下来，随后将两个数字转换为正数。

在被除数大于除数的前提下，用被除数前去除数，记录次数。

但这个过程其实就是模，而且已经被禁止了，所以我们需要思考第二种思路。

我们注意到得到的商可以用二进制的形式表达，因此在每一轮中，我们将除数乘以2（由于不能进行除法操作，
所以我们实际上要使用移位操作符），直到找到最大的数字，让结果小于除数。

我们用被除数减去除数乘以这个值，得到的余数进入下一轮循环。这个值被累加到总结果中，循环在余数小于除数
时结束。

### 代码实现

###### c++
```c++
class Solution {
public:
    int divide(int dividend, int divisor) {
        bool positive = (dividend < 0 == divisor < 0);
        long a = abs(dividend);
        long b = abs(divisor);
        unsigned int result = 0;
        while (a >= b) {
            short q = 0;
            while (a > (b<<(q+1))) { q++; }
            result += (1<<q);
            a -= (b<<q);
        }
        if (result == (1<<31) && positive) return INT_MAX;
        return positive ? result : -result;
    }
};
```