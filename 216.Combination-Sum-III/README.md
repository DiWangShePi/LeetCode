# 216. Combination Sum III

### Description

Find all valid combinations of k numbers that sum up to n such that the following conditions are true:

- Only numbers 1 through 9 are used.
- Each number is used at most once.
Return a list of all possible valid combinations. The list must not contain the same combination twice, and the combinations may be returned in any order.

### Solution

范围是数字0到9，递归就行了。

### Implementation

###### c++

```c++
class Solution {
private:
    void dfs(vector<vector<int>>& result, int k, int n, int index, int sum, vector<int>& current) {
        if (sum == n && k == 0) {
            result.push_back(current);
            return;
        }
        if (sum > n || index > 9) return;

        // do not take this number
        dfs(result, k, n, index+1, sum, current);

        // take this number
        current.push_back(index);
        dfs(result, k - 1, n, index + 1, sum + index, current);
        current.pop_back();
    }

public:
    vector<vector<int>> combinationSum3(int k, int n) {
        vector<vector<int>> result;
        vector<int> current;
        dfs(result, k, n, 1, 0, current);
        return result;
    }
};
```
