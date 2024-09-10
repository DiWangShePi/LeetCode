# 39. Combination Sum

### 题目描述

给定一个列表candidates，其中的数字各不相同。给定目标值target，返回candidates中数字组合的一个列表，使得每一个组合的数字之和为目标值。列表中的
组合可以以任意顺序。

candidates中的数字可以在组合中出现不限次，只要两个组合中有至少一个数字出现的频率不一致，两个组合就是不一样的。

输入的测试样本确保组合的总数目少于150次。

**示例：**

```
Input: candidates = [2,3,6,7], target = 7
Output: [[2,2,3],[7]]
Explanation:
2 and 3 are candidates, and 2 + 2 + 3 = 7. Note that 2 can be used multiple times.
7 is a candidate, and 7 = 7.
These are the only two combinations.
```

```
Input: candidates = [2,3,5], target = 8
Output: [[2,2,2,2],[2,3,3],[3,5]]
```

```
Input: candidates = [2], target = 1
Output: []
```

### 题目解析

深度优先遍历。由于每一个数字是可能重复的，所以我们在每一次探索中，考虑当前数字重复一遍，和下一个位置的数字。
若和为目标值，则将当前组合加入结果并返回。
若大于目标值，则直接返回（因为输入数字没有负数）。
若小于目标值，则遍历所有可能性，继续向下探索。

### 代码实现

```c++
#include <vector>

class Solution {
public:
    vector<vector<int>> combinationSum(vector<int>& candidates, int target) {
        vector<vector<int>> res;
        vector<int> tmp;
        helper(candidates, target, res, tmp, 0, 0);
        return res;
    }

    void helper(vector<int>& candidates, int target, vector<vector<int>>& result, vector<int>& temp, int value, int pos) {
        if (value == target) {
          vector<int> candidateCopy = temp;
          result.push_back(candidateCopy);
          return;
        }

        if ( value > target || pos > candidates.size()-1 ) return;

        temp.push_back(candidates[pos]);
        helper(candidates, target, result, temp, value+candidates[pos], pos);
        temp.pop_back();
        helper(candidates, target, result, temp, value, pos+1);
    }
};
```