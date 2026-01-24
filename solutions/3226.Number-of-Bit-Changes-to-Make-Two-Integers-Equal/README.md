# 3226. Number of Bit Changes to Make Two Integers Equal

**Tags:** Bitwise operations

### Description

You are given two positive integers n and k.

You can choose any bit in the binary representation of n that is equal to 1 and change it to 0.

Return the number of changes needed to make n equal to k. If it is impossible, return -1.

### Example

###### Example I

> Input: n = 13, k = 4
> Output: 2
> Explanation:
> Initially, the binary representations of n and k are n = (1101)2 and k = (0100)2.
> We can change the first and fourth bits of n. The resulting integer is n = (0100)2 = k.

###### Example II

> Input: n = 21, k = 21
> Output: 0
> Explanation:
> n and k are already equal, so no changes are needed.

###### Example III

> Input: n = 14, k = 13
> Output: -1
> Explanation:
> It is not possible to make n equal to k.

### Solution

遍历两个数字的二进制表示，逐位检查。

```c++
class Solution {
public:
    int minChanges(int n, int k) {
        int res = 0;
        while (n > 0 || k > 0) {
            if ((n & 1) == 0 && (k & 1) == 1) {
                return -1;
            }
            if ((n & 1) == 1 && (k & 1) == 0) {
                res++;
            }
            n >>= 1;
            k >>= 1;
        }
        return res;
    }
};
```
