# 3890. Integers With Multiple Sum of Two Cubes

**Tags:** Math, Hash Table

### Description

You are given an integer n.

An integer x is considered good if there exist at least two distinct pairs (a, b) such that:

- a and b are positive integers.
- a <= b
- x = a^3 + b^3
Return an array containing all good integers less than or equal to n, sorted in ascending order.

### Example

###### Example I

> Input: n = 4104
> Output: [1729,4104]
> Explanation:
> Among integers less than or equal to 4104, the good integers are:
> 1729: 13 + 123 = 1729 and 93 + 103 = 1729.
> 4104: 23 + 163 = 4104 and 93 + 153 = 4104.
> Thus, the answer is [1729, 4104].

###### Example II

> Input: n = 578
> Output: []
> Explanation:
> There are no good integers less than or equal to 578, so the answer is an empty array.

### Solution

暴力尝试

```c++
class Solution {
public:
    vector<int> findGoodIntegers(int n) {
        vector<int> ans;
        unordered_map<int, int> dict;

        for (long long i = 1; i * i * i < n; i++) {
            long long i3 = i * i * i;
            for (long long j = 1; j < i; j++) {
                long long c = i3 + j * j * j;
                if (c > n) continue;

                dict[c]++;
                if (dict[c] == 2) ans.push_back((int)c);
            }
        }
        sort(ans.begin(), ans.end());
        return ans;
    }
};
```
