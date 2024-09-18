# 46. Permutations

### 题目描述

给定一个由不同整数组成的数组 nums，返回所有可能的排列。您可以按任何顺序返回答案。

**示例：**

```
Input: nums = [1,2,3]
Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
```

```
Input: nums = [0,1]
Output: [[0,1],[1,0]]
```

### 题目解析

递归，每次选择一个数字，到底时将列表加入答案中。

### 代码实现

###### c++

```c++
class Solution {
public:
    vector<vector<int>> permute(vector<int>& nums) {
        vector<vector<int>> ans;
        vector<int> temp;
        vector<bool> contains(nums.size(), false);
        for (int i = 0; i < nums.size(); i++) {
            temp.push_back(nums[i]);
            contains[i] = true;

            deep(1, temp, contains, nums, ans);

            temp.pop_back();
            contains[i] = false;
        }
        return ans;
    }

    void deep(int index, vector<int> temp, vector<bool>& contains, vector<int>& nums, vector<vector<int>>& ans) {
        if (index == nums.size()) {
            ans.push_back(temp);
            return;
        }

        for (int i = 0; i < nums.size(); i++) {
            if (contains[i]) continue;

            temp.push_back(nums[i]);
            contains[i] = true;

            deep(index+1, temp, contains, nums, ans);

            temp.pop_back();
            contains[i] = false;
        }
    }
};
```