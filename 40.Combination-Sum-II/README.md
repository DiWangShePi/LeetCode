# 40. Combination Sum II

### 题目描述

给定一组候选数字（candidate）和一个目标数字（target），找出候选数字之和等于目标的所有唯一组合。

候选数字中的每个数字在组合中只能使用一次。

注意：解决方案集不得包含重复的组合。

### 题目解析

递归，与上一题一致。

### 代码实现

###### c++

```c++
#include <vector>
#include <algorithm>

class Solution {
public:
    vector<vector<int>> combinationSum2(vector<int>& candidates, int target) {
        vector<vector<int>> res;
        vector<int> tmp;

        std::sort(candidates.begin(), candidates.end());
        helper(candidates, target, res, tmp, 0, 0);
        return res;
    }

    void helper(vector<int>& candidates, int target, vector<vector<int>>& result, vector<int>& temp, int value, int pos) {
        if (value == target) {
            vector<int> candidateCopy = temp;
            result.push_back(candidateCopy);
            return;
        }

        if ( value > target || pos > candidates.size() - 1) return;
        for (int i = pos; i < candidates.size(); ++i) {
            if (i > pos && candidates[i] == candidates[i - 1]) continue;
            if (candidates[i] > target) break;
            temp.push_back(candidates[i]);
            helper(candidates, target, result, temp, value + candidates[i], i + 1);
            temp.pop_back();
        }
    }
};
```