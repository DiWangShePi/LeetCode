# 201. Bitwise AND of Numbers Range

### Description

Given two integers left and right that represent the range [left, right], return the bitwise AND of all numbers in this range, inclusive.

### Solution

我一开始想到的解法就和题目的要求一样直接，遍历区域内的所有数字，将他们重复的做与操作。出乎意料的，这样竟然就会超时。

所以我们进一步考虑优化后的解法。题目要求我们做与操作，而对于与操作而言，只有当两个数字均为1的时候，最终得到的结果才为1，否则即为0。

因此，我们考虑找到left和right所具有的共同前缀，这样会是这段区域内所有数字的公共前缀。
我们知道这段前缀的数字会在与操作下保留下来，而剩下的所有数字都会变为0，因为我们会遇到0和1。

于是就得到了一个优化的解法。

### Implementation

###### c++

```c++
class Solution {
public:
    int rangeBitwiseAnd(int left, int right) {
        int shift = 0;
        while (left < right) {
            left >>= 1;
            right >>= 1;
            shift++;
        }
        return left << shift;
    }
};
```
