# 2429. Minimize XOR

**Tags:** Bitwise Operation

### Description

Given two positive integers num1 and num2, find the positive integer x such that:

- x has the same number of set bits as num2, and
- The value x XOR num1 is minimal.
Note that XOR is the bitwise XOR operation.

Return the integer x. The test cases are generated such that x is uniquely determined.

The number of set bits of an integer is the number of 1's in its binary representation.

### Example

###### Example I

> Input: num1 = 3, num2 = 5
> Output: 3
> Explanation:
> The binary representations of num1 and num2 are 0011 and 0101, respectively.
> The integer 3 has the same number of set bits as num2, and the value 3 XOR 3 = 0 is minimal.

###### Example II

> Input: num1 = 1, num2 = 12
> Output: 3
> Explanation:
> The binary representations of num1 and num2 are 0001 and 1100, respectively.
> The integer 3 has the same number of set bits as num2, and the value 3 XOR 1 = 2 is minimal.

### Solution

我们可以任意构造数字，只要个数和 num2 一致即可，所以我们先数一遍 num2 有多少个 1。
- 如果比 num1 多，那么 num1 中的每一个 1 都可以被异或消掉，然后对于 num1 二进制中的 0，我们从低到高将其变为 1。
- 如果比 num1 少，那么 num1 中不是每一个 1 都可以被消掉，为了让结果最小，我们从高到低将其变为 0。

```c++
class Solution {
public:
    int minimizeXor(int num1, int num2) {
        int c1 = 0, c2 = 0;
        for (int i = 0; i < 32; i++) {
            if ((num1 & (1 << i)) != 0) c1++;
            if ((num2 & (1 << i)) != 0) c2++;
        }

        int an = 0;
        if (c1 < c2) {
            int c = c2 - c1, an = 0;
            for (int i = 0; i < 32; i++) {
                if ((num1 & (1 << i)) == 0) {
                    int r = (1 << i);
                    an |= r;
                    
                    c--;
                    if (c == 0) return an | num1;
                }
            }
        } else {
            for (int i = 31; i > -1; i--) {
                if ((num1 & (1 << i)) != 0) {
                    int r = (1 << i);
                    an |= r;
                    
                    c2--;
                    if (c2 == 0) return an;
                }
            }
        }
        return -1;
    }
};
```
