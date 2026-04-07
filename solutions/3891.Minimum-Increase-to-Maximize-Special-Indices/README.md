# 3891. Minimum Increase to Maximize Special Indices

**Tags:** Dynamic Programming

### Description

You are given an integer array nums of length n.

An index i (0 < i < n - 1) is special if nums[i] > nums[i - 1] and nums[i] > nums[i + 1].

You may perform operations where you choose any index i and increase nums[i] by 1.

Your goal is to:

- Maximize the number of special indices.
- Minimize the total number of operations required to achieve that maximum.
Return an integer denoting the minimum total number of operations required.

### Example 

###### Example I

> Input: nums = [1,2,2]
> Output: 1
> Explanation:​​​​​​​
> Start with nums = [1, 2, 2].
> Increase nums[1] by 1, array becomes [1, 3, 2].
> The final array is [1, 3, 2] has 1 special index, which is the maximum achievable.
> It is impossible to achieve this number of special indices with fewer operations. Thus, the answer is 1.

###### Example II

> Input: nums = [2,1,1,3]
> Output: 2
> Explanation:​​​​​​​
> Start with nums = [2, 1, 1, 3].
> Perform 2 operations at index 1, array becomes [2, 3, 1, 3].
> The final array is [2, 3, 1, 3] has 1 special index, which is the maximum achievable. Thus, the answer is 2.

###### Example III

> Input: nums = [5,2,1,4,3]
> Output: 4
> Explanation:​​​​​​​​​​​​​​​​​​​​​
> Start with nums = [5, 2, 1, 4, 3].
> Perform 4 operations at index 1, array becomes [5, 6, 1, 4, 3].
> The final array is [5, 6, 1, 4, 3] has 2 special indices, which is the maximum achievable. Thus, the answer is 4.​​​​​​​

### Solution

当数组长度为奇数时，可能的答案只有一种，直接计算即可。

当数组长度为偶数时，总的个数固定，但可能是错开的。

比如

> [12,23,13,17,21,3]

我们可以将 23 和 17 变为 PEAK，也可以是 23 和 21 变为 PEAK，也可以是 13 和 21 变为 PEAK。三种得到的 PEAK 个数是一样的，但第二种的开销最小，为 0。

```c++
class Solution {
public:
    long long minIncrease(vector<int>& nums) {
        int n = nums.size();
        vector<long long> left(n, 0), right(n, 0);

        auto cost = [&](int i) -> long long {
            int c = max(nums[i - 1], nums[i + 1]);
            return max(0, c + 1 - nums[i]);
        };

        for (int i = 1; i < n - 1; i++) {
            left[i] = cost(i);
            if (i - 2 >= 1) left[i] += left[i - 2];
        }
        if (n % 2 == 1) return left[n - 2];

        for (int i = n - 2; i >= 1; i--) {
            right[i] = cost(i);
            if (i + 2 <= n - 2) right[i] += right[i + 2];
        }

        long long res = min(left[n - 3], right[2]);
        for (int i = 1; i <= n - 5; i += 2) {
            res = min(res, left[i] + right[i + 3]);
        }
        return res;
    }
};
```
