# 2048. Next Greater Numerically Balanced Number

**Tags:** Math

### Description

An integer x is numerically balanced if for every digit d in the number x, there are exactly d occurrences of that digit in x.

Given an integer n, return the smallest numerically balanced number strictly greater than n.

### Example

###### Example I

> Input: n = 1
> Output: 22
> Explanation: 
> 22 is numerically balanced since:
> - The digit 2 occurs 2 times. 
> It is also the smallest numerically balanced number strictly greater than 1.

###### Example II

> Input: n = 1000
> Output: 1333
> Explanation: 
> 1333 is numerically balanced since:
> - The digit 1 occurs 1 time.
> - The digit 3 occurs 3 times. 
> It is also the smallest numerically balanced number strictly greater than 1000.
> Note that 1022 cannot be the answer because 0 appeared more than 0 times.

###### Example III

> Input: n = 3000
> Output: 3133
> Explanation: 
> 3133 is numerically balanced since:
> - The digit 1 occurs 1 time.
> - The digit 3 occurs 3 times.
> It is also the smallest numerically balanced number strictly greater than 3000.

### Solution

我热爱暴力

```c++
class Solution {
public:
    int nextBeautifulNumber(int n) {
        int i = n + 1;
        for ( ; i <= 1224444; i++) {
            if (check(i)) break;
        }
        return i;
    }

private:
    bool check(int n) {
        vector<int> dict(10, 0);
        while (n > 0) {
            int t = n % 10;
            n /= 10;
            dict[t]++;
        }

        for (int i = 0; i < 10; i++) {
            if (dict[i] > 0 && dict[i] != i) return false;
        }
        return true;
    }
};
```

我也热爱打表

```c++
class Solution {
public:
    const vector<int> balance {
        1, 22, 122, 212, 221, 333, 1333, 3133, 3313, 3331, 4444,
        14444, 22333, 23233, 23323, 23332, 32233, 32323, 32332,
        33223, 33232, 33322, 41444, 44144, 44414, 44441, 55555,
        122333, 123233, 123323, 123332, 132233, 132323, 132332,
        133223, 133232, 133322, 155555, 212333, 213233, 213323,
        213332, 221333, 223133, 223313, 223331, 224444, 231233,
        231323, 231332, 232133, 232313, 232331, 233123, 233132,
        233213, 233231, 233312, 233321, 242444, 244244, 244424,
        244442, 312233, 312323, 312332, 313223, 313232, 313322,
        321233, 321323, 321332, 322133, 322313, 322331, 323123,
        323132, 323213, 323231, 323312, 323321, 331223, 331232,
        331322, 332123, 332132, 332213, 332231, 332312, 332321,
        333122, 333212, 333221, 422444, 424244, 424424, 424442,
        442244, 442424, 442442, 444224, 444242, 444422, 515555,
        551555, 555155, 555515, 555551, 666666, 1224444
    };

    int nextBeautifulNumber(int n) {
        return *upper_bound(balance.begin(), balance.end(), n);
    }
};
```
