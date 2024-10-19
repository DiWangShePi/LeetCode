# 78. Subsets

### 题目描述

给定一个不包含重复数字的列表，返回所有可能的子列表。

返回的答案可以以任意顺序，答案中不能包含重复的结果。

**示例：**

```
Input: nums = [1,2,3]
Output: [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
```

```
Input: nums = [0]
Output: [[],[0]]
```

### 题目解析

深度搜索，每一个元素有在和不在两种状态，搜到底部了返回。

### 代码实现

###### c++

```c++
class Solution {
public:
    vector<vector<int>> subsets(vector<int>& nums) {
        vector<vector<int>> result;
        vector<int> current;
        dfs(0, nums, current, result);
        return result;
    }

    void dfs(int depth, vector<int>& nums, vector<int>& current, vector<vector<int>>& result) {
        if (depth == nums.size) {
            result.push_back(current);
            return;
        }

        current.push_back(nums[depth]);
        dfs(depth+1, nums, current, result);
        current.pop_back();

        dfs(depth+1, nums, current, result);
    }
};
```