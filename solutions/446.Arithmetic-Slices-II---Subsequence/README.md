# 446. Arithmetic Slices II - Subsequence

### Decription

Given an integer array nums, return the number of all the arithmetic subsequences of nums.

A sequence of numbers is called arithmetic if it consists of at least three elements and if the difference between any two consecutive elements is the same.

- For example, [1, 3, 5, 7, 9], [7, 7, 7, 7], and [3, -1, -5, -9] are arithmetic sequences.
- For example, [1, 1, 2, 5, 7] is not an arithmetic sequence.
A subsequence of an array is a sequence that can be formed by removing some elements (possibly none) of the array.

- For example, [2,5,10] is a subsequence of [1,2,1,2,4,1,5,10].
The test cases are generated so that the answer fits in 32-bit integer.

### Example 

###### Example I

> Input: nums = [2,4,6,8,10]
> Output: 7
> Explanation: All arithmetic subsequence slices are:
> [2,4,6]
> [4,6,8]
> [6,8,10]
> [2,4,6,8]
> [4,6,8,10]
> [2,4,6,8,10]
> [2,6,10]

###### Example II

> Input: nums = [7,7,7,7,7]
> Output: 16
> Explanation: Any subsequence of this array is arithmetic.

### Solution

知道尾项和公差即可知道一个等差数列（当然还有长度）。我们可以采用动态规划，定义f[i][j]为尾项为nums[i]，公差为j的等差数列的个数（长度为2的）。

用二重循环枚举所有数字对，在j相同时，f[i][j]可以被累加到第二项上，且由于当前的两个数字同样构成等差，因此结果还要再加一。

```c++
class Solution {
public:
    int numberOfArithmeticSlices(vector<int>& nums) {
        int an = 0;
        vector<unordered_map<long long, int>> dp(nums.size());
        for (int i = 0; i < nums.size(); i++) {
            for (int j = 0; j < i; j++) {
                long long d = 1LL * nums[i] - nums[j];
                int cnt = dp[j].count(d) != 0 ? dp[j][d] : 0;

                an += cnt;
                dp[i][d] += cnt + 1;
            }
        }
        return an;
    }
};
```
