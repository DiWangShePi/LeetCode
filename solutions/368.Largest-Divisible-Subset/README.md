# 368. Largest Divisible Subset

### Description

Given a set of distinct positive integers nums, return the largest subset answer such that every pair (answer[i], answer[j]) of elements in this subset satisfies:

- answer[i] % answer[j] == 0, or
- answer[j] % answer[i] == 0
If there are multiple solutions, return any of them.

### Example

###### Example I

```
Input: nums = [1,2,3]
Output: [1,2]
Explanation: [1,3] is also accepted.
```

###### Example II

```
Input: nums = [1,2,4,8]
Output: [1,2,4,8]
```

### Solution

根据整除关系的传递性（若 a | b 且 b | c，则 a | c），可以推导出：

若整数 a 是整除子集 S₁ 的最小整数 b 的约数（即 a | b），则可将 a 加入 S₁ 得到更大的整除子集；

若整数 c 是整除子集 S₂ 的最大整数 d 的倍数（即 d | c），则可将 c 加入 S₂ 得到更大的整除子集。

基于此，使用动态规划数组 dp 记录以每个元素结尾的最大整除子集长度。对于每个 nums[i]，检查其前方所有 nums[j]，若 nums[i] % nums[j] == 0，则更新 dp[i]。同时维护全局最大子集长度和对应的最大值。最后逆向遍历数组，根据 dp 值和整除关系收集结果子集。

```c++
class Solution {
public:
    vector<int> largestDivisibleSubset(vector<int>& nums) {
        int n = nums.size();
        sort(nums.begin(), nums.end());

        vector<int> dp(n, 1);
        int maxSize = 0, maxVal;
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < i; j++) {
                if (nums[i] % nums[j] == 0) dp[i] = max(dp[i], dp[j] + 1);
            }

            if (dp[i] > maxSize) {
                maxSize = dp[i];
                maxVal = nums[i];
            }
        }

        vector<int> an;
        for (int i = n - 1; i > -1; i--) {
            if (dp[i] == maxSize && maxVal % nums[i] == 0) {
                maxSize--;
                maxVal = nums[i];
                an.push_back(nums[i]);
            }
        }
        return an;
    }
};
```
