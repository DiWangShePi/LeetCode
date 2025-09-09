# 476. Number Complement

### Description

The complement of an integer is the integer you get when you flip all the 0's to 1's and all the 1's to 0's in its binary representation.

- For example, The integer 5 is "101" in binary and its complement is "010" which is the integer 2.
Given an integer num, return its complement.

### Example 

###### Example I

> Input: num = 5
> Output: 2
> Explanation: The binary representation of 5 is 101 (no leading zero bits), and its complement is 010. So you need to output 2.

###### Example II

> Input: num = 1
> Output: 0
> Explanation: The binary representation of 1 is 1 (no leading zero bits), and its complement is 0. So you need to output 0.

### Solution

生成一个和给的num一样长，且全为1的二进制数字，然后做异或处理。

```c++
class Solution {
public:
    int findComplement(int num) {
        int mask = 1;
        while (mask < num) {
            mask = (mask << 1) | 1;
        }
        return mask ^ num;
    }
};
```
