# 372. Super Pow

### Description

Your task is to calculate a^b mod 1337 where a is a positive integer and b is an extremely large positive integer given in the form of an array.

### Example 

###### Example I

> Input: a = 2, b = [3]
> Output: 8

###### Example II

> Input: a = 2, b = [1,0]
> Output: 1024

###### Example III

> Input: a = 1, b = [4,3,3,8,5,2]
> Output: 1

### Solution

秦九韶算法

```c++
class Solution {
    const int MOD = 1337;

    int pow(int x, int n) {
        int res = 1;
        while (n) {
            if (n % 2) {
                res = (long) res * x % MOD;
            }
            x = (long) x * x % MOD;
            n /= 2;
        }
        return res;
    }

public:
    int superPow(int a, vector<int> &b) {
        int ans = 1;
        for (int e: b) {
            ans = (long) pow(ans, 10) * pow(a, e) % MOD;
        }
        return ans;
    }
};
```