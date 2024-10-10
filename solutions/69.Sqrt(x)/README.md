# 69. Sqrt(x)

### 题目描述

给定一个非负整数 x，返回 x 的平方根向下舍入到最接近的整数。返回的整数也应该是非负的。

您不得使用任何内置指数函数或运算符。

例如，不要在 c++ 中使用 pow(x, 0.5) 或在 python 中使用 x ** 0.5。

```
Input: x = 4
Output: 2
Explanation: The square root of 4 is 2, so we return 2.
```

```
Input: x = 8
Output: 2
Explanation: The square root of 8 is 2.82842..., and since we round it down to the nearest integer, 2 is returned.
```

### 题目解析

若x为0或1，直接返回x.

划定区间[1,x]，二分查找目标数字n，使得n的平方小于x，且n+1的平方大于x。

### 代码实现

###### c++

```c++
class Solution {
public:
    int mySqrt(int x) {
        if (x == 0) return 0;
        if (x == 1) return 1;

        int l = 1, r = x;
        long mid = 0;
        while (l <= r) {
            mid = l + (r - l) / 2;
            if (mid * mid > x) {
                r = mid - 1;
            } else if (mid * mid < x) {
                l = mid + 1;
            } else {
                return mid;
            }
        }
        return r;
    }
};
```