# 415. Add Strings


### Description

Given two non-negative integers, num1 and num2 represented as string, return the sum of num1 and num2 as a string.

You must solve the problem without using any built-in library for handling large integers (such as BigInteger). You must also not convert the inputs to integers directly.

### Example 

###### Example I

> Input: num1 = "11", num2 = "123"
> Output: "134"

###### Example II

> Input: num1 = "456", num2 = "77"
> Output: "533"

###### Example III

> Input: num1 = "0", num2 = "0"
> Output: "0"

### Solution

遍历，累加

```c++
class Solution {
public:
    string addStrings(string num1, string num2) {
        if (num1.size() < num2.size()) return addStrings(num2, num1);

        string an;
        an.reserve(num1.size() + 1); 
        int add = 0;
        int i = num1.size() - 1, j = num2.size() - 1;

        for (; j >= 0; i--, j--) {
            int c = (num1[i] - '0') + (num2[j] - '0') + add;
            add = c / 10;
            an.push_back('0' + (c % 10));
        }
        for (; i >= 0; i--) {
            int c = (num1[i] - '0') + add;
            add = c / 10;
            an.push_back('0' + (c % 10));
        }
        if (add != 0) an.push_back('0' + add);

        reverse(an.begin(), an.end());
        return an;
    }
};
```
