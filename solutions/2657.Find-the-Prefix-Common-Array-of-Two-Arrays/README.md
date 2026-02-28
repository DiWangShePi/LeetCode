# 2657. Find the Prefix Common Array of Two Arrays

**Tags:** Dict, Bitwise Operation

### Description

You are given two 0-indexed integer permutations A and B of length n.

A prefix common array of A and B is an array C such that C[i] is equal to the count of numbers that are present at or before the index i in both A and B.

Return the prefix common array of A and B.

A sequence of n integers is called a permutation if it contains all integers from 1 to n exactly once.

### Example 

###### Example I

> Input: A = [1,3,2,4], B = [3,1,2,4]
> Output: [0,2,3,4]
> Explanation: At i = 0: no number is common, so C[0] = 0.
> At i = 1: 1 and 3 are common in A and B, so C[1] = 2.
> At i = 2: 1, 2, and 3 are common in A and B, so C[2] = 3.
> At i = 3: 1, 2, 3, and 4 are common in A and B, so C[3] = 4.

###### Example II

> Input: A = [2,3,1], B = [3,1,2]
> Output: [0,1,3]
> Explanation: At i = 0: no number is common, so C[0] = 0.
> At i = 1: only 3 is common in A and B, so C[1] = 1.
> At i = 2: 1, 2, and 3 are common in A and B, so C[2] = 3.

### Solution

简单的解法是依次添加检查，每记录 A 的一个数，查一遍 B 检查有多少个相同的。用字典记录是否出现过。

```c++
class Solution {
public:
    vector<int> findThePrefixCommonArray(vector<int>& A, vector<int>& B) {
        unordered_map<int, int> dict;
        vector<int> an;
        int n = A.size();

        for (int i = 0; i < n; i++) {
            dict[A[i]]++;
            int c = 0;
            for (int j = 0; j <= i; j++) {
                if (dict.count(B[j]) != 0)
                    c += dict[B[j]];
            }
            an.push_back(c);
        }
        return an;
    }
};
```

不过，由于题目保证了数组的长度不会超过 50，所以我们可以用位运算来记录出现过的数字。

```c++
class Solution {
public:
    vector<int> findThePrefixCommonArray(vector<int>& a, vector<int>& b) {
        uint64_t p = 0, q = 0;
        for (int i = 0; i < a.size(); i++) {
            p |= 1ULL << a[i];
            q |= 1ULL << b[i];
            a[i] = popcount(p & q);
        }
        return a;
    }
};
```
