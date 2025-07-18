# 412. Fizz Buzz

### Description

Given an integer n, return a string array answer (1-indexed) where:

- answer[i] == "FizzBuzz" if i is divisible by 3 and 5.
- answer[i] == "Fizz" if i is divisible by 3.
- answer[i] == "Buzz" if i is divisible by 5.
- answer[i] == i (as a string) if none of the above conditions are true.

### Example 

###### Example I

> Input: n = 3
> Output: ["1","2","Fizz"]

###### Example II

> Input: n = 5
> Output: ["1","2","Fizz","4","Buzz"]

###### Example III

> Input: n = 15
> Output: ["1","2","Fizz","4","Buzz","Fizz","7","8","Fizz","Buzz","11","Fizz","13","14","FizzBuzz"]

### Solution

遍历检测，按要求填入字符串

```c++
class Solution {
public:
    vector<string> fizzBuzz(int n) {
        vector<string> an(n);
        for (int i = 1; i <= n; i++) {
            if (i % 3 == 0 && i % 5 == 0) an[i - 1] = "FizzBuzz";
            else if (i % 3 == 0) an[i - 1] = "Fizz";
            else if (i % 5 == 0) an[i - 1] = "Buzz";
            else an[i - 1] = to_string(i);
        }
        return an;
    }
};
```
