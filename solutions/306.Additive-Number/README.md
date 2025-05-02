# 306. Additive Number

### Description

An additive number is a string whose digits can form an additive sequence.

A valid additive sequence should contain at least three numbers. Except for the first two numbers, each subsequent number in the sequence must be the sum of the preceding two.

Given a string containing only digits, return true if it is an additive number or false otherwise.

Note: Numbers in the additive sequence cannot have leading zeros, so sequence `1`, `2`, `03` or `1`, `02`, `3` is invalid.

### Example 

###### Example I

```
Input: "112358"
Output: true
Explanation: 
The digits can form an additive sequence: 1, 1, 2, 3, 5, 8. 
1 + 1 = 2, 1 + 2 = 3, 2 + 3 = 5, 3 + 5 = 8
```

###### Example II

```
Input: "199100199"
Output: true
Explanation: 
The additive sequence is: 1, 99, 100, 199. 
1 + 99 = 100, 99 + 100 = 199
```

### Solution

当前两位决定了的时候，整个数列该是怎么样，就已经确定了，剩下要做的就是去检查给定的答案是否符合。

因此枚举前两位的所有可能，再检查即可。

```c++
class Solution {
public:
    bool isAdditiveNumber(string num) {
        int n = num.size();
        if (n < 3) return false;
        
        for (int i = 1; i <= n/2; i++) {
            for (int j = 1; max(i, j) <= n - i - j; j++) {
                string s1 = num.substr(0, i);
                string s2 = num.substr(i, j);

                if ((s1.size() > 1 && s1[0] == '0') || (s2.size() > 1 && s2[0] == '0')) {
                    continue;
                }
                
                long long first = stoll(s1);
                long long second = stoll(s2);
                
                if (check(first, second, num.substr(i + j))) {
                    return true;
                }
            }
        }
        return false;
    }

private:
    bool check(long long first, long long second, string num) {
        if (num.empty()) return true;
        
        long long sum = first + second;
        string sumStr = to_string(sum);
        
        if (num.substr(0, sumStr.size()) != sumStr) {
            return false;
        }
        
        return check(second, sum, num.substr(sumStr.size()));
    }
};
```
