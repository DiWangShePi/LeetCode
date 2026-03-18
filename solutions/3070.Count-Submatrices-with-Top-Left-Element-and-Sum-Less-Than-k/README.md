# 3070. Count Submatrices with Top-Left Element and Sum Less Than k

**Tags:** Prefix Sum

### Description

You are given a 0-indexed integer matrix grid and an integer k.

Return the number of submatrices that contain the top-left element of the grid, and have a sum less than or equal to k.

### Example

###### Example I

![](./example1.png)

> Input: grid = [[7,6,3],[6,6,1]], k = 18
> Output: 4
> Explanation: There are only 4 submatrices, shown in the image above, that contain the top-left element of grid, and have a sum less than or equal to 18.

###### Example II

![](./example21.png)

> Input: grid = [[7,2,9],[1,5,0],[2,6,6]], k = 20
> Output: 6
> Explanation: There are only 6 submatrices, shown in the image above, that contain the top-left element of grid, and have a sum less than or equal to 20.

### Solution

计算矩阵的前缀和，统计有多少个满足条件的。

```c++
class Solution {
public:
    int countSubmatrices(vector<vector<int>>& grid, int k) {
        int n = grid.size(), m = grid[0].size();
        vector<vector<int>> sums(n, vector<int>(m, 0));

        sums[0][0] = grid[0][0];
        int ans = sums[0][0] <= k ? 1 : 0;
        for (int i = 1; i < n; i++) {
            sums[i][0] = sums[i - 1][0] + grid[i][0];
            if (sums[i][0] <= k) ans++;
        }
        for (int i = 1; i < m; i++) {
            sums[0][i] = sums[0][i - 1] + grid[0][i];
            if (sums[0][i] <= k) ans++;
        }
        for (int i = 1; i < n; i++) {
            for (int j = 1; j < m; j++) {
                sums[i][j] = sums[i - 1][j] + sums[i][j - 1] + grid[i][j] - sums[i - 1][j - 1];
                if (sums[i][j] <= k) ans++;
            }
        }
        return ans;
    }
};
```
