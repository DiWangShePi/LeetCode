# 47. Permutations II

### 题目描述

给定一个数组nums，其中可能包含重复的数字，返回所有可能的排列。您可以按任何顺序返回答案。

**示例：**

```
Input: nums = [1,1,2]
Output:
[[1,1,2],
 [1,2,1],
 [2,1,1]]
```

```
Input: nums = [1,2,3]
Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]
```

### 题目解析

使用c++的next_permutation函数

### 代码实现

###### c++

```c++
class Solution {
public:
vector<vector<int>> permuteUnique(vector<int>& nums) {
vector<vector<int>> ans;
sort(nums.begin(),nums.end());

        ans.push_back(nums);

        while(next_permutation(nums.begin(),nums.end())) {
            ans.push_back(nums);
        }
        return ans;
    }
};
```
