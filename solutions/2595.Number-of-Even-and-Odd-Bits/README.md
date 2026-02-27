# 2595. Number of Even and Odd Bits

**Tags:** Bitwise Operation

### Description

You are given a positive integer n.

Let even denote the number of even indices in the binary representation of n with value 1.

Let odd denote the number of odd indices in the binary representation of n with value 1.

Note that bits are indexed from right to left in the binary representation of a number.

Return the array [even, odd].

### Example

###### Example I

> Input: n = 50
> Output: [1,2]
> Explanation:
> The binary representation of 50 is 110010.
> It contains 1 on indices 1, 4, and 5.

###### Example II

> Input: n = 2
> Output: [0,1]
> Explanation:
> The binary representation of 2 is 10.
> It contains 1 only on index 1.

### Solution

数一遍

```c++
class Solution {
public:
    vector<int> evenOddBit(int n) {
        int odd = 0, even = 0;
        for (int i = 0; i < 32; i++) {
            if (i % 2 == 0 && (n & (1 << i)) != 0) even++;
            else if (i % 2 == 1 && (n & (1 << i)) != 0) odd++;
        }
        return {even, odd};
    }
};
```
