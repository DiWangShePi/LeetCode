# 43. Multiply Strings

### 题目描述

给定两个非负整数num1和num2（以字符串的形式）。计算两者的乘积，也以字符串的形式返回。

不可以使用现有的如BigInteger库或者将整个数字转换为数字。

### 题目解析

初始化一个数组，遍历num1和num2两个字符串，逐个计算元素的乘积，并将其加入到保存的数组中。

### 代码实现

###### c++

```c++
class Solution {
public:
    string multiply(string num1, string num2) {

         // handle edge-case where the product is 0
        if (num1 == "0" || num2 == "0") return "0";
        
        // num1.size() + num2.size() == max no. of digits 
        vector<int> num(num1.size() + num2.size(), 0);
        
        // build the number by multiplying one digit at the time
        // Multiplication will occur just like the normal multiplication that learned do in the class 5th
        // Mulitplying last digit of first number with every digit of second number while maintaining the carry and going so on
        for (int i = num1.size() - 1; i >= 0; i--) {
            for (int j = num2.size() - 1; j >= 0; j--) {
                // For this solution let's say num[i+j+1] = digit 1 and num[i+j] = digit 2
                num[i + j + 1] += (num1[i] - '0') * (num2[j] - '0'); // Calculating digit 1
               // cout<<num[i+j+1]<<" ";  for the dry run check
                num[i + j] += num[i + j + 1] / 10;// Maintaining the carry of the digit 1
               //  cout<<num[i+j]<<" ";
                num[i + j + 1] %= 10; // digit 1 after giving up the carry to next digit 2
               // cout<<num[i+j+1]<<" ";
            }
        }
        
        // skip leading 0's in the number array 
        int i = 0;
        while (i < num.size() && num[i] == 0) i++;
        
        // transofrm the vector to a string 
        string res = "";
        while (i < num.size()) res.push_back(num[i++] + '0');
        
        return res;
    }
};
```