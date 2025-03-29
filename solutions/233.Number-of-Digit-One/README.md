# 233. Number of Digit One

### Description

Given an integer n, count the total number of digit 1 appearing in all non-negative integers less than or equal to n.

### Solution

> 我热爱暴力，暴力让我成了我，我的LeetCode记录成了我的记录。但是这一次，我却让暴力解丢了脸...

我们可以通过逐位分析的方法来高效计算从 0 到 n 之间数字 "1" 出现的总次数。对于每一位（个位、十位、百位等），我们将 n 分解为 高位部分（high）、当前位（cur） 和 低位部分（low），然后计算当前位上 "1" 的贡献：如果 cur == 0，贡献是 high * d；如果 cur == 1，贡献是 high * d + (low + 1)；如果 cur >= 2，贡献是 (high + 1) * d。遍历所有数位，最终累加得出答案。

类似于LeetCode 172。

### Implementation

###### c++

```c++
class Solution {
public:
    int countDigitOne(int n) {
        int count = 0;
        long long d = 1; 

        while (d <= n) {
            long long high = n / (10 * d);
            int cur = (n / d) % 10;
            int low = n % d;

            if (cur == 0) {
                count += high * d;
            } else if (cur == 1) {
                count += high * d + (low + 1);
            } else {
                count += (high + 1) * d;
            }

            d *= 10; 
        }
        return count;
    }
};
```
