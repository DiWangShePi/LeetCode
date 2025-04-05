# 264. Ugly Number II

### Description

An ugly number is a positive integer whose prime factors are limited to 2, 3, and 5.

Given an integer n, return the nth ugly number.

### Solution

用最小合成方式不断生成下一个丑数，使用三个指针分别表示乘2、乘3、乘5的下一个候选值。

```c++
class Solution {
public:
    int nthUglyNumber(int n) {
        vector<int> result{1};
        int ugly2 = 0, ugly3 = 0, ugly5 = 0;
        int next2, next3, next5;
        for (int i = 0; i < n; i++) {
            next2 = result[ugly2] * 2;
            next3 = result[ugly3] * 3;
            next5 = result[ugly5] * 5;
            int current = min(next2, min(next3, next5));

            if (current == next2) ugly2++;
            if (current == next3) ugly3++;
            if (current == next5) ugly5++;

            result.push_back(current);
        }
        return result[n - 1];
    }
};
```
