# 66. Plus One

### 题目描述

给定一个大整数，表示为整数数组digits，其中每个数字[i]是该整数的第i位。数字按从左到右的顺序从最高有效到最低有效。大整数不包含任何前导0。将大整数加1并返回结果的数字数组。

**题目描述**

```
Input: digits = [1,2,3]
Output: [1,2,4]
Explanation: The array represents the integer 123.
Incrementing by one gives 123 + 1 = 124.
Thus, the result should be [1,2,4].
```

```
Input: digits = [4,3,2,1]
Output: [4,3,2,2]
Explanation: The array represents the integer 4321.
Incrementing by one gives 4321 + 1 = 4322.
Thus, the result should be [4,3,2,2].
```

```
Input: digits = [9]
Output: [1,0]
Explanation: The array represents the integer 9.
Incrementing by one gives 9 + 1 = 10.
Thus, the result should be [1,0].
```

### 题目解析

按要求实现即可，注意进位


### 代码实现

###### c++

```c++
class Solution {
public:
    vector<int> plusOne(vector<int>& digits) {
        int carry = 1, current = 0;
        for (int i = digits.size() - 1 ; i > -1; i--) {
            current = digits[i] + carry;
            digits[i] = current % 10;
            carry = current / 10;
        }

        if (carry != 0) digits.insert(digits.begin(), carry);
        return digits;
    }
};
```