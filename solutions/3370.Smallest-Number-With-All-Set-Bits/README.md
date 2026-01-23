# 3370. Smallest Number With All Set Bits

### Description

You are given a positive number n.

Return the smallest number x greater than or equal to n, such that the binary representation of x contains only set bits

### Example

###### Example I

> Input: n = 5
> Output: 7
> Explanation:
> The binary representation of 7 is "111".

###### Example II

> Input: n = 10
> Output: 15
> Explanation:
> The binary representation of 15 is "1111".

###### Example III

> Input: n = 3
> Output: 3
> Explanation:
> The binary representation of 3 is "11".

### Solution

位运算，从 0 开始，每次新加一个 1，直到得到的数字大于指定值。

```c++
class Solution {
public:
    int smallestNumber(int n) {
        int an = 0, pos = 0;
        while (an < n) an |= (1 << pos++);
        return an;
    }
};
```
