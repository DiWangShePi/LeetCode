# 491. Non-decreasing Subsequences

### Description

Given an integer array nums, return all the different possible non-decreasing subsequences of the given array with at least two elements. You may return the answer in any order.

### Example 

###### Example I

> Input: nums = [4,6,7,7]
> Output: [[4,6],[4,6,7],[4,6,7,7],[4,7],[4,7,7],[6,7],[6,7,7],[7,7]]

###### Example II

> Input: nums = [4,4,3,2,1]
> Output: [[4,4]]

### Solution

我们还是先考虑暴力的方式：枚举所有长度为2且符合要求的数字，随后遍历数组，如果当前元素可以被添加到一个现在的数组中，那么就以此构造一个新的数组并添加到数组中。

> 感觉每次这样做，一看到题脑子里想的全是暴力

```c++
class Solution {
public:
    vector<vector<int>> findSubsequences(vector<int>& nums) {
        vector<vector<int>> ans;
        int n = nums.size();
        for (int i = 0; i < n; i++) {
            for (int j = i + 1; j < n; j++) {
                if (nums[j] >= nums[i]) ans.push_back({i, j});
            }
        }

        for (int i = 2; i < n; i++) {
            int t = ans.size();

            for (int j = 0; j < t; j++) {
                vector<int> current = ans[j];
                int l = current.back();

                if (i > l && nums[i] >= nums[l]) {
                    current.push_back(i);
                    ans.push_back(current);
                }
            }
        }

        set<vector<int>> an;
        for (const auto& a : ans) {
            vector<int> t;
            for (int index : a) {
                t.push_back(nums[index]);
            } 
            an.insert(move(t));
        }
        return vector<vector<int>>(an.begin(), an.end());
    }
};
```

下面给出一个优化后的解法，不需要最后一步的转换，搜索过程中保证数列序列值的递增；在每一层去重，不需要最后去重。

```c++
class Solution {
public:
    vector<vector<int>> findSubsequences(vector<int>& nums) {
        vector<vector<int>> result;
        int n = nums.size();
        
        vector<int> current;
        backtrack(nums, 0, current, result);
        return result;
    }

private:
    void backtrack(const vector<int>& nums, int start, vector<int>& current, vector<vector<int>>& result) {
        if (current.size() >= 2) {
            result.push_back(current);
        }
        
        unordered_set<int> used; 
        
        for (int i = start; i < nums.size(); i++) {
            if (used.find(nums[i]) != used.end()) continue;
            
            if (current.empty() || nums[i] >= current.back()) {
                used.insert(nums[i]);
                current.push_back(nums[i]);
                backtrack(nums, i + 1, current, result);
                current.pop_back();
            }
        }
    }
};
```
