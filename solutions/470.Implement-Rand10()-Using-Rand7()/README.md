# 470. Implement Rand10() Using Rand7()

**Tags:** Math

### Description

Given the API rand7() that generates a uniform random integer in the range [1, 7], write a function rand10() that generates a uniform random integer in the range [1, 10]. You can only call the API rand7(), and you shouldn't call any other API. Please do not use a language's built-in random API.

Each test case will have one internal argument n, the number of times that your implemented function rand10() will be called while testing. Note that this is not an argument passed to rand10().

### Example 

###### Example I

> Input: n = 1
> Output: [2]

###### Example II

> Input: n = 2
> Output: [2,8]

###### Example III

> Input: n = 3
> Output: [3,8,10]

### Solution

组合小范围的随机数来均匀的映射大范围的随机数。

```c++
// The rand7() API is already defined for you.
// int rand7();
// @return a random integer in the range 1 to 7

class Solution {
public:
    int rand10() {
        int an = (rand7() - 1) * 7 + rand7();
        while (an > 40) an = (rand7() - 1) * 7 + rand7();
        return (an - 1) / 4 + 1;
    }
};
```
