# 73. Set Matrix Zeroes

### 题目描述

给定一个m乘n的整数矩阵，如果其中一个元素是0，则将其所在的行和列都变为0。

你需要在给定的矩阵上完成。

**示例：**

```
Input: matrix = [[1,1,1],[1,0,1],[1,1,1]]
Output: [[1,0,1],[0,0,0],[1,0,1]]
```

```
Input: matrix = [[0,1,2,0],[3,4,5,2],[1,3,1,5]]
Output: [[0,0,0,0],[0,4,5,0],[0,3,1,0]]
```

### 题目解析

首先，我们不能在遍历的过程中遇到0就将该行和列置为0，这会让后续遍历过程出现问题。

因此，一个O(mn)的解决方案是额外创建一个等大的矩阵，用于指示对应元素是否需要变为0。

或者我们可以尝试优化这个方案，如果遇到了一个0，则将该行和该列纳入到要置为0的指示中，这是一个O(m+n)的矩阵。

在该方案上再进一步，我们发现我们需要的只是一个该行或者该列是否需要置为0的指示符，这可以用该行或者该列的第一个元素完成。
若某个元素为0，则将该行和该列的第一个元素置为0。这第一个元素原本是否为0并不重要。
唯一需要额外注意的便是第一行和第一列，这其中的元素是否为0需要额外确认一遍。

### 代码实现

###### c++

```c++
class Solution {
public:
    void setZeroes(vector<vector<int>>& matrix) {
        int row = 1, column = 1;
        int m = matrix.size(), n = matrix[0].size();
        
        for (int i = 0; i < m; i++) {
            if (matrix[i][0] == 0) column = 0;
        }
        for (int i = 0; i < n; i++) {
            if (matrix[0][i] == 0) row = 0;
        }

        for (int i = 1; i < m; i++) {
            for (int j = 1; j < n; j++) {
                if (matrix[i][j] == 0) {
                    matrix[i][0] = 0;
                    matrix[0][j] = 0;
                }
            }
        }

        for (int i = 1; i < m; i++) {
            if (matrix[i][0] == 0) {
                fill(matrix[i].begin() + 1, matrix[i].end(), 0);  
            }
        }

        for (int j = 1; j < n; j++) {
            if (matrix[0][j] == 0) {
                for (int i = 1; i < m; i++) {
                    matrix[i][j] = 0;  
                }
            }
        }

        if (row == 0) {
            fill(matrix[0].begin(), matrix[0].end(), 0);
        }
        if (column == 0) {
            for (int i = 0; i < m; i++) {
                matrix[i][0] = 0;
            }
        }
    }
};

```