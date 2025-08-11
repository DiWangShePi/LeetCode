# 440. K-th Smallest in Lexicographical Order

### Description

Given two integers n and k, return the kth lexicographically smallest integer in the range [1, n].

### Example

###### Example I

> Input: n = 13, k = 2
> Output: 10
> Explanation: The lexicographical order is [1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9], so the second smallest number is 10.

###### Example II

> Input: n = 1, k = 1
> Output: 1

### Solution

我们还是先给出一个暴力的解法：模拟深度优先搜索的过程，生成数字1到n之间的前k个数字

```c++
class Solution {
public:
    int findKthNumber(int n, int k) {
        int index = 1;
        long long current = 1;
        while (index < k) {
            index++;

            if (current * 10 <= n) {
                current *= 10;
            } else {
                if (current >= n) current /= 10;

                current += 1;
                while (current % 10 == 0) current /= 10;
            }
        }
        return current;
    }
};
```

这一解法的复杂度是O(k)的，不符合题目要求，因此我们要尝试优化一些的版本。

我们发现，一个数字沿字典序展开的树，其包含的满足条件的数字个数是可以计算出来的。

```c++
class Solution {
public:
    int findKthNumber(int n, int k) {
        long long current = 1;
        k--;
        while (k > 0) {
            int step = getSteps(current, n);
            if (step <= k) {
                k -= step;
                current++;
            } else {
                current *= 10;
                k--;
            }
        }
        return current;
    }

private:
    int getSteps(int curr, long n) {
        int step = 0;
        long start = curr, last = curr;
        while (start <= n) {
            step += min(last, n) - start + 1;
            start = start * 10;
            last = last * 10 + 9;
        }
        return step;
    }
};
```
