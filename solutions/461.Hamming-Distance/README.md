# 461. Hamming Distance

### Description

The Hamming distance between two integers is the number of positions at which the corresponding bits are different.

Given two integers x and y, return the Hamming distance between them.

### Example 

###### Example I

> Input: x = 1, y = 4
> Output: 2
> Explanation:
> 1   (0 0 0 1)
> 4   (0 1 0 0)
>        ↑   ↑
> The above arrows point to positions where the corresponding bits are different.

###### Example II

> Input: x = 3, y = 1
> Output: 1

### Solution

先做异或操作，然后检查有多少个位是1

```c++
class Solution {
public:
    int hammingDistance(int x, int y) {
        int c = x ^ y;
        int count = 0;
        for (int i = 31; i > -1; i--) {
            if (c & ( 1 << i)) count++;
        }
        return count;
    }
};
```
