# 537. Complex Number Multiplication

### Description

A complex number can be represented as a string on the form "real+imaginaryi" where:

- real is the real part and is an integer in the range [-100, 100].
- imaginary is the imaginary part and is an integer in the range [-100, 100].
- i2 == -1.
Given two complex numbers num1 and num2 as strings, return a string of the complex number that represents their multiplications.

### Example

###### Example I

> Input: num1 = "1+1i", num2 = "1+1i"
> Output: "0+2i"
> Explanation: (1 + i) * (1 + i) = 1 + i2 + 2 * i = 2i, and you need convert it to the form of 0+2i.

###### Example II

> Input: num1 = "1+-1i", num2 = "1+-1i"
> Output: "0+-2i"
> Explanation: (1 - i) * (1 - i) = 1 + i2 - 2 * i = -2i, and you need convert it to the form of 0+-2i.

### Solution

按要求实现即可

```c++
class Solution {
public:
    string complexNumberMultiply(string num1, string num2) {
        int plus1 = num1.find('+');
        int real1 = stoi(num1.substr(0, plus1));
        int imag1 = stoi(num1.substr(plus1 + 1, num1.length() - plus1 - 2));
        
        int plus2 = num2.find('+');
        int real2 = stoi(num2.substr(0, plus2));
        int imag2 = stoi(num2.substr(plus2 + 1, num2.length() - plus2 - 2));
        
        int real_part = real1 * real2 - imag1 * imag2;
        int imag_part = real1 * imag2 + imag1 * real2;
        
        return to_string(real_part) + "+" + to_string(imag_part) + "i";
    }
};
```
