# 1486. XOR Operation in an Array

**Tags:** Bitwise operations

### Description

You are given an integer n and an integer start.

Define an array nums where nums[i] = start + 2 * i (0-indexed) and n == nums.length.

Return the bitwise XOR of all elements of nums.

### Example

###### Example I

> Input: n = 5, start = 0
> Output: 8
> Explanation: Array nums is equal to [0, 2, 4, 6, 8] where (0 ^ 2 ^ 4 ^ 6 ^ 8) = 8.
> Where "^" corresponds to bitwise XOR operator.

###### Example II

> Input: n = 4, start = 3
> Output: 8
> Explanation: Array nums is equal to [3, 5, 7, 9] where (3 ^ 5 ^ 7 ^ 9) = 8.

### Solution

按要求实现即可

```c++
class Solution {
public:
    int xorOperation(int n, int start) {
        int an = start;
        for (int i = 1; i < n; i++) {
            an ^= start + 2 * i;
        }
        return an;
    }
};
```