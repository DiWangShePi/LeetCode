# 85. Maximal Rectangle

### Problem Description

Given a rows x cols binary matrix filled with 0's and 1's, find the largest rectangle containing only 1's and return its area.

**Example: **

```
Input: matrix = [["1","0","1","0","0"],["1","0","1","1","1"],["1","1","1","1","1"],["1","0","0","1","0"]]
Output: 6
Explanation: The maximal rectangle is shown in the above picture.
```

```
Input: matrix = [["0"]]
Output: 0
```

```
Input: matrix = [["1"]]
Output: 1
```

### Problem Solution

我们首先考虑暴力的做法，枚举每一个可能的矩形，检查其是否符合条件，随后计算面积。这太暴力了。

接下来，考虑到我们需要的是一种快速知道当前点所能构成的合法矩形的长和高的矩形（为了方便并保证一致性，我们假设当前点作为矩形的右下角），
回想上一题的方式，我们可以对每一行，计算其左边连续1的个数，作为当前行的特殊值。检查这个值和该元素上方（到第一个小于这个值的元素），即为高。

计算左边连续值的方式同样参考上一题，可以有O(mn)复杂度的解法。在获得每一个矩阵值的上边界和下边界时，可以用一样的方式。

### Code Implemption

###### c++

```c++
class Solution {
public:
    int maximalRectangle(vector<vector<char>>& matrix) {
        int m = matrix.size();
        if (m == 0) {
            return 0;
        }
        int n = matrix[0].size();
        vector<vector<int>> left(m, vector<int>(n, 0));

        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                if (matrix[i][j] == '1') {
                    left[i][j] = (j == 0 ? 0: left[i][j - 1]) + 1;
                }
            }
        }

        int ret = 0;
        for (int j = 0; j < n; j++) {
            vector<int> up(m, 0), down(m, 0);

            stack<int> stk;
            for (int i = 0; i < m; i++) {
                while (!stk.empty() && left[stk.top()][j] >= left[i][j]) {
                    stk.pop();
                }
                up[i] = stk.empty() ? -1 : stk.top();
                stk.push(i);
            }
            stk = stack<int>();
            for (int i = m - 1; i >= 0; i--) {
                while (!stk.empty() && left[stk.top()][j] >= left[i][j]) {
                    stk.pop();
                }
                down[i] = stk.empty() ? m : stk.top();
                stk.push(i);
            }

            for (int i = 0; i < m; i++) {
                int height = down[i] - up[i] - 1;
                int area = height * left[i][j];
                ret = max(ret, area);
            }
        }
        return ret;
    }
};
```
