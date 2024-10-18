# 77. Combinations

### 题目描述

给定数字n和k，返回从[1,n]中选择k个数字的所有可能结果。

你的答案可以以任意顺序。

**示例：**

```
Input: n = 4, k = 2
Output: [[1,2],[1,3],[1,4],[2,3],[2,4],[3,4]]
Explanation: There are 4 choose 2 = 6 total combinations.
Note that combinations are unordered, i.e., [1,2] and [2,1] are considered to be the same combination.
```

```
Input: n = 1, k = 1
Output: [[1]]
Explanation: There is 1 choose 1 = 1 total combination.
```

### 题目解析

每个元素有两种状态，在已有答案中和不在已有答案中。深度搜索这一状态树，当答案长度满足时将当前结果
列入最终答案，搜索结束后返回。

### 代码实现

###### c++

```c++
class Solution {
public:
    vector<vector<int>> combine(int n, int k) {
        vector<vector<int>> result;
        vector<int> current;
        dfs(1, n, k, current, result);
        return result;
    }

    void dfs(int index, int n, int k, vector<int>& current, vector<vector<int>>& result) {
        if (current.size() == k) {
            result.push_back(current);
            return;
        }

        if (current.size() + (n - index + 1) < k) {
            return;
        }

        // add current number
        current.push_back(index);
        dfs(index+1, n, k, current, result);
        current.pop_back();
            
        // do not add current number
        dfs(index+1, n, k, current, result);
    }
};
```