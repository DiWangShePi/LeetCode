# 400. Nth Digit

### Description

Given an integer n, return the nth digit of the infinite integer sequence [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, ...].

### Example 

###### Example I

> Input: n = 3
> Output: 3

###### Example II

> Input: n = 11
> Output: 0
> Explanation: The 11th digit of the sequence 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, ... is a 0, which is part of the number 10.

### Solution

我们还是先给出一个暴力的解法，检测我们对于题意理解的正确与否

```c++
class Solution {
public:
    int findNthDigit(int n) {
        int i = 1;
        string current = "";
        while (n > 0) {
            current = to_string(i++);

            if (n <= current.size()) break;
            n -= current.size();
        }
        return current[n - 1] - '0';
    }
};
```

接下来我们尝试时间复杂度更低一些的算法，我们发现，数字的长度其实是有迹可循的，
- 比如1到9都是1位，有9个数字
- 10到99都是两位，有90个数字
- 100到999都是三位，有900个数字 
- ...

由此，我们可以更快速的降低n的值，以锁定目标的数字。

```c++
class Solution {
public:
    int findNthDigit(int n) {
        int digit = 1;      
        long start = 1;     
        long count = 9;     

        while (n > count) {
            n -= count;
            digit++;
            start *= 10;
            count = 9 * start * digit;
        }

        long num = start + (n - 1) / digit;

        string s = to_string(num);
        return s[(n - 1) % digit] - '0';
    }
};
```
