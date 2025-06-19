# 376. Wiggle Subsequence

### Description

A wiggle sequence is a sequence where the differences between successive numbers strictly alternate between positive and negative. The first difference (if one exists) may be either positive or negative. A sequence with one element and a sequence with two non-equal elements are trivially wiggle sequences.

- For example, [1, 7, 4, 9, 2, 5] is a wiggle sequence because the differences (6, -3, 5, -7, 3) alternate between positive and negative.
- In contrast, [1, 4, 7, 2, 5] and [1, 7, 4, 5, 5] are not wiggle sequences. The first is not because its first two differences are positive, and the second is not because its last difference is zero.
A subsequence is obtained by deleting some elements (possibly zero) from the original sequence, leaving the remaining elements in their original order.

Given an integer array nums, return the length of the longest wiggle subsequence of nums.

### Example 

###### Example I

```
Input: nums = [1,7,4,9,2,5]
Output: 6
Explanation: The entire sequence is a wiggle sequence with differences (6, -3, 5, -7, 3).
```

###### Example II

```
Input: nums = [1,17,5,10,13,15,10,5,16,8]
Output: 7
Explanation: There are several subsequences that achieve this length.
One is [1, 17, 10, 13, 10, 16, 8] with differences (16, -7, 3, -3, 6, -8).
```

###### Example III

```
Input: nums = [1,2,3,4,5,6,7,8,9]
Output: 2
```

### Solution

O(n^2)的思路和第300题差不多，采用动态规划，对于范围是[0, i]的子序列，遍历[0, j]检查是否可以使得[0, i]更长。

```c++
class Solution {
public:
    int wiggleMaxLength(vector<int>& nums) {
        int n = nums.size();
        if (n == 1) return 1;

        vector<int> dp(n, 1);
        vector<int> positive(n, 0);
        if (nums[1] - nums[0] != 0) {
            positive[1] = nums[1] - nums[0] > 0 ? 1 : -1;
            dp[1] = 2;
        }

        int an = max(1, dp[1]);
        for (int i = 2; i < n; i++) {
            for (int j = 0; j < i; j++) {
                // update when up
                if (nums[i] > nums[j] && (positive[j] == -1 || positive[j] == 0)) {
                    if (dp[i] < dp[j] + 1) {
                        dp[i] = dp[j] + 1;
                        positive[i] = 1;
                    }
                }

                // update when down
                if (nums[i] < nums[j] && (positive[j] == 1 || positive[j] == 0)) {
                    if (dp[i] < dp[j] + 1) {
                        dp[i] = dp[j] + 1;
                        positive[i] = -1;
                    }
                }
            }

            an = max(an, dp[i]);
        }
        return an;
    }
};
```

> 优化解法

```c++
class Solution {
public:
    int wiggleMaxLength(vector<int>& nums) {
        int n = nums.size();
        if (n <= 1) return n;

        vector<int> up(n, 1), down(n, 1);

        for (int i = 1; i < n; ++i) {
            for (int j = 0; j < i; ++j) {
                if (nums[i] > nums[j]) {
                    up[i] = max(up[i], down[j] + 1);
                } else if (nums[i] < nums[j]) {
                    down[i] = max(down[i], up[j] + 1);
                }
            }
        }

        return max(up[n - 1], down[n - 1]);
    }
};
```

但随后我们可以发现，当我们在考虑当前数字和上一个数字构成的序列是否延长了上升或下降序列时，我们并不关系之前的序列在那里结束，我们知道他一定给可以接上去，所以我们只需要维护两个值，不需要数组，也不需要遍历检查。

```c++
class Solution {
public:
    int wiggleMaxLength(vector<int>& nums) {
        if (nums.size() < 2) return nums.size();

        int up = 1, down = 1;

        for (int i = 1; i < nums.size(); ++i) {
            if (nums[i] > nums[i - 1]) up = down + 1;
            if (nums[i] < nums[i - 1]) down = up + 1;
        }
        return max(up, down);
    }
};
```
