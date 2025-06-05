# 343. Integer Break

### Description

Given an integer n, break it into the sum of k positive integers, where k >= 2, and maximize the product of those integers.

Return the maximum product you can get.

### Example

###### Example I

```
Input: n = 2
Output: 1
Explanation: 2 = 1 + 1, 1 × 1 = 1.
```

###### Example II

```
Input: n = 10
Output: 36
Explanation: 10 = 3 + 3 + 4, 3 × 3 × 4 = 36.
```

### Solution

动态规划

```c++
class Solution {
public:
    int integerBreak(int n) {
        vector<long long> result(n + 1, 0);
        result[1] = 1;

        return dfs(n, result);
    }

private:    
    long long dfs(long long n, vector<long long>& result) {
        if (result[n] != 0) return max(result[n], n);

        long long max_result = 0, current;
        for (int i = 1; i <= n; i++) {
            current = i * dfs(n - i, result);
            if (current > max_result) max_result = current;
        }
        result[n] = max_result;
        return max_result;
    }
};
```
