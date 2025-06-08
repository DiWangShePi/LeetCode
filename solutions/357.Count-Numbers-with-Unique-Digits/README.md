# 357. Count Numbers with Unique Digits

### Description

Given an integer n, return the count of all numbers with unique digits, x, where 0 <= x < 10^n.

### Example 

###### Example I

```
Input: n = 2
Output: 91
Explanation: The answer should be the total numbers in the range of 0 ≤ x < 100, excluding 11,22,33,44,55,66,77,88,99
```

###### Example II

```
Input: n = 0
Output: 1
```

### Solution

暴力的方式是遍历范围内的数字，检查每一个是否是有重复数字的

```c++
class Solution {
public:
    int countNumbersWithUniqueDigits(int n) {
        int an = 0;
        int range = pow(10, n);
        vector<int> dict(10, 0);
        for (int i = 0; i < range; i++) {
            string current = to_string(i);
            if (check(current, dict)) an++;
            fill(dict.begin(), dict.end(), 0);
        }
        return an;
    }

private:
    bool check(string num, vector<int>& dict) {
        for (int i = 0; i < num.size(); i++) {
            int p = num[i] - '0';
            if (dict[p] == 1) return false;
            dict[p] = 1;
        }
        return true;
    }
};
```

我们也可以数学组合原理来计算具有唯一数字的数字数量。对于n位数，第一位有9种选择（1-9），后续每位可选数字依次递减（避免重复），总数量是各位数情况的累加。当n=0时只有数字0，n>10时最多考虑10位数（更多位数必然重复）。

```c++
class Solution {
public:
    int countNumbersWithUniqueDigits(int n) {
        if (n == 0) return 1;
        if (n > 10) n = 10; 
        
        int total = 10;
        int uniqueDigits = 9;
        
        for (int i = 2; i <= n; ++i) {
            uniqueDigits *= (10 - (i - 1)); 
            total += uniqueDigits;
        }
        
        return total;
    }
};
```
