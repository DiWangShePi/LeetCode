# 18. 4Sum

给定长度为n的整数数组，返回一个独立元组列表[nums[a], nums[b], nums[c], nums[d]]，满足一下条件：

1. 0 <= a, b, c, d < n
2. 构成的元组不重复
3. nums[a] + nums[b] + nums[c] + nums[d] == target

你可以以任意顺序返回数组

**示例：**

```
Input: nums = [1,0,-1,0,-2,2], target = 0
Output: [[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]
```

```
Input: nums = [2,2,2,2,2], target = 8
Output: [[2,2,2,2]]
```

### 题目解析

在三数之和的基础上，再加一层for循环

### 代码实现

###### c++

```c++
class Solution {
public:
    vector<vector<int>> fourSum(vector<int>& nums, int target) {
        sort(nums.begin(), nums.end());
        set<vector<int>> results;

        int n = nums.size();
        for (int i = 0; i < n - 3; ++i) {
            if (i > 0 && nums[i] == nums[i - 1]) continue; // Skip duplicates
            for (int j = i + 1; j < n - 2; ++j) {
                if (j > i + 1 && nums[j] == nums[j - 1]) continue; // Skip duplicates
                int left = j + 1, right = n - 1;
                while (left < right) {
                    long long total = static_cast<long long>(nums[i]) + nums[j] + nums[left] + nums[right];
                    if (total == target) {
                        results.insert({nums[i], nums[j], nums[left], nums[right]});
                        ++left;
                        --right;
                        while (left < right && nums[left] == nums[left - 1]) ++left; // Skip duplicates
                        while (left < right && nums[right] == nums[right + 1]) --right; // Skip duplicates
                    } else if (total < target) {
                        ++left;
                    } else {
                        --right;
                    }
                }
            }
        }

        return vector<vector<int>>(results.begin(), results.end());
    }
};
```