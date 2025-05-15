# 338. Counting Bits

### Description

Given an integer n, return an array ans of length n + 1 such that for each i (0 <= i <= n), ans[i] is the number of 1's in the binary representation of i.

### Example 

###### Example I

```
Input: n = 2
Output: [0,1,1]
Explanation:
0 --> 0
1 --> 1
2 --> 10
```

###### Example II

```
Input: n = 5
Output: [0,1,1,2,1,2]
Explanation:
0 --> 0
1 --> 1
2 --> 10
3 --> 11
4 --> 100
5 --> 101
```

### Solution

n - 1 会把从右往左第一个 1 变成 0，并把右边所有的 0 变成 1。 与 n 按位与后，那个最右边的 1 就被清除掉了，其他位保持不变。

```c++
class Solution {
public:
    vector<int> countBits(int n) {
        vector<int> ans;
        for (int i = 0; i <= n; i++) ans.push_back(count(i));
        return ans;
    }

private:
    int count(int n) {
        int an = 0;
        while (n > 0) {
            n &= n - 1;
            an++;
        }
        return an;
    }
};
```