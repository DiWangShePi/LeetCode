# 221. Maximal Square

### Description

Given an m x n binary matrix filled with 0's and 1's, find the largest square containing only 1's and return its area.

### Solution

动态规划：
- 如果一个`[i][j]`位置下的元素值为0，则以该元素为右下角，所能构建的满足条件的正方形边大小必定为0。
- 如果一个`[i][j]`位置下的元素值为1，则以该元素为右下角，所能构建的满足条件的正方形大小为该元素上方，左方和左上方三个位置的最小值加一。

### Implementation

###### c++

```c++
class Solution {
public:
    int maximalSquare(vector<vector<char>>& matrix) {
        int m = matrix.size();
        int n = matrix[0].size();
        vector<vector<int>> square(m, vector<int>(n, 0));

        int result = 0;
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                if (matrix[i][j] == '1') {
                    if (i == 0 || j == 0) square[i][j] = 1;
                    else {
                        square[i][j] = min(square[i - 1][j - 1], min(square[i - 1][j], square[i][j - 1])) + 1;
                    }
                    result = max(square[i][j], result);
                }
            }
        }

        return result * result;
    }
};
```
