# 441. Arranging Coins

### Description

You have n coins and you want to build a staircase with these coins. The staircase consists of k rows where the ith row has exactly i coins. The last row of the staircase may be incomplete.

Given the integer n, return the number of complete rows of the staircase you will build.

### Example 

###### Example I

![](./arrangecoins1-grid.jpg)

> Input: n = 5
> Output: 2
> Explanation: Because the 3rd row is incomplete, we return 2.

###### Example II

![](./arrangecoins2-grid.jpg)

> Input: n = 8
> Output: 3
> Explanation: Because the 4th row is incomplete, we return 3.

### Solution

暴力的方式：遍历累加数字，直到达到或者超过n

```c++
class Solution {
public:
    int arrangeCoins(int n) {
        int i = 1;
        while (n > 0) n -= i++;
        return n == 0 ? i - 1 : i - 2;
    }
};
```

聪明的方式：我们知道梯子阶数累加是一个递增数列，我们可以二分查找一个合适的层数k，k为累加和大于等于n的最小数字。

```c++
class Solution {
public:
    int arrangeCoins(int n) {
        int left = 1, right = n;
        while (left < right) {
            int mid = (right - left + 1) / 2 + left;
            if ((long long) mid * (mid + 1) <= (long long) 2 * n) {
                left = mid;
            } else {
                right = mid - 1;
            }
        }
        return left;
    }
};
```

但既然都上数学了，其实也可以直接求解。

```c++
class Solution {
public:
    int arrangeCoins(int n) {
        return (int) ((sqrt((long long) 8 * n + 1) - 1) / 2);
    }
};
```
