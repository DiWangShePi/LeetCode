# 1009. Complement of Base 10 Integer

**Tags:** Bitwise operations

### Description

The complement of an integer is the integer you get when you flip all the 0's to 1's and all the 1's to 0's in its binary representation.

- For example, The integer 5 is "101" in binary and its complement is "010" which is the integer 2.
Given an integer n, return its complement.

### Example

###### Example I

> Input: n = 5
> Output: 2
> Explanation: 5 is "101" in binary, with complement "010" in binary, which is 2 in base-10.

###### Example II

> Input: n = 7
> Output: 0
> Explanation: 7 is "111" in binary, with complement "000" in binary, which is 0 in base-10.

###### Example III

> Input: n = 10
> Output: 5
> Explanation: 10 is "1010" in binary, with complement "0101" in binary, which is 5 in base-10.

### Solution

构造与给定数字长度相同的全 1 数字，然后做异或。

```c++
class Solution {
public:
    int bitwiseComplement(int n) {
        if (n == 0) return 1;
        
        int mask = 1;
        while (mask < n) {
            mask = (mask << 1) | 1;  
        }
        return mask ^ n;  
    }
};
```

或者

```c++
class Solution {
public:
    int bitwiseComplement(int n) {
        if (n == 0) return 1;
        
        int an = 0, t = 1;
        while (n != 0) {
            if ((n & 1) == 0) an |= t;
            t <<= 1;
            n >>= 1;
        }
        return an;
    }
};
```
