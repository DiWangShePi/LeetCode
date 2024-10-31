# 90. Subsets II

### Question Description

Given an integer array nums that may contain duplicates, return all possible 
subsets (the power set).

The solution set must not contain duplicate subsets. Return the solution in any order.

**Example: **

```
Input: nums = [1,2,2]
Output: [[],[1],[1,2],[1,2,2],[2],[2,2]]
```

```
Input: nums = [0]
Output: [[],[0]]
```

### Solution

深度搜索，每一个值有在或不在两种状态。到底部后结果加入到一个集合中（确保不重复）。

### Code Implemption

###### c++

```c++
class Solution {
public:
    vector<vector<int>> subsetsWithDup(vector<int>& nums) {

        sort(nums.begin(), nums.end());
        
        vector<vector<int>> ans;
        vector<int> subset;
        int i = 0;

        dfs(ans, subset, nums, i);

        return ans;
    }

    void dfs(vector<vector<int>>& ans,vector<int>& subset, vector<int>& nums, int i){

        if(i >= nums.size()){
            ans.push_back(subset);
            return;
        }

        subset.push_back(nums[i]);
        dfs(ans, subset, nums, i + 1);
        subset.pop_back();

        while(i + 1 < nums.size() && nums[i] == nums[i+1]){
            i++;
        }

        dfs(ans, subset, nums, i + 1);
    }
};
```