# 172. Factorial Trailing Zeroes

### Description

Given an integer n, return the number of trailing zeroes in n!.

Note that n! = n * (n - 1) * (n - 2) * ... * 3 * 2 * 1.

### Solution

我们实际上只需要找能构成10的数字，比如2和5，在整个计算过程中会出现多少次。这不单只统计2和5，还要考虑如10这样的数字，它是由2和5构成的。

我们进一步考虑到，由于2出现的次数远比5多，而必须要一个2和一个5才能构成一个10，所以我们可以只考虑5出现的次数。

### Implementation

###### c++

```c++
class Solution {
public:
    int trailingZeroes(int n) {
        int count = 0;
        while (n >= 5) {
            n /= 5;
            count += n;
        }
        return count;
    }
};
```
