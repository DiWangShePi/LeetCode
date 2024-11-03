# 96. Unique Binary Search Trees

### Question Description

Given an integer n, return the number of structurally unique BST's (binary search trees) which has exactly n nodes of unique values from 1 to n.

**Example: **

```
Input: n = 3
Output: 5
```

```
Input: n = 1
Output: 1
```

### Solution

与上一题一样，最后返回数组的长度

### Code Implemption

###### c++

```c++
class Solution {
public:
    int numTrees(int n) {
        std::vector<int> G(n + 1, 0);
        G[0] = 1;
        G[1] = 1;

        for (int i = 2; i <= n; ++i) {
            for (int j = 1; j <= i; ++j) {
                G[i] += G[j - 1] * G[i - j];
            }
        }

        return G[n];
    }
};
```

一个有意思的事情，是这个题目的范围是1<=n<=19，因此其实我们也可以穷举，将答案写死在代码里。

```c++
class Solution {
public:
    int numTrees(int n) {
        if (n == 1) return 1;
        else if (n == 2) return 2;
        else if (n == 3) return 5;
        else if (n == 4) return 14;
        else if (n == 5) return 42;
        else if (n == 6) return 132;
        else if (n == 7) return 429;
        else if (n == 8) return 1430;
        else if (n == 9) return 4862;
        else if (n == 10) return 16796;
        else if (n == 11) return 58786;
        else if (n == 12) return 208012;
        else if (n == 13) return 742900;
        else if (n == 14) return 2674440;
        else if (n == 15) return 9694845;
        else if (n == 16) return 35357670;
        else if (n == 17) return 129644790;
        else if (n == 18) return 477638700;
        return 1767263190;
    }
};
```