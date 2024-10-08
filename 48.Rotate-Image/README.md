# 48. Rotate Image

### 题目描述

给定一个n✖n的2D矩阵，将其顺时针旋转九十度。

你必须在给定的数据结构上完成旋转，这意味着你不能创建一个新的矩阵完成这项工作

### 题目解析

我们要做的其实是将矩阵中四个数字交换位置。左上的换到右上，右上的换到右下，右下的换到左下，左下的换到左上。

### 代码实现

###### c++

```c++
class Solution {
public:
    void rotate(vector<vector<int>>& matrix) {
        int row = matrix.size();
        for(int i=0;i<row; i++){
            for(int j=0; j<=i;j++){
                swap(matrix[i][j], matrix[j][i]);
            }
        }
        for(int i=0;i<row;i++){
            reverse(matrix[i].begin(), matrix[i].end());
        }
    }
};
```

not recommend

```c++
class Solution {
public:
    void rotate(vector<vector<int>>& matrix) {
        int n = matrix.size() % 2 != 0 ? (matrix.size() - 1) / 2 : (matrix.size() + 1) / 2;
        for (int j = 0; j < n; j++) {
             for (int i = 0; i < n; i++) {
                int t_l = matrix[j][i];
                int d_l = matrix[matrix.size() - 1 - i][j];
                int d_r = matrix[matrix.size() - 1 - j][matrix.size() - 1 - i];
                int t_r = matrix[i][matrix.size() - 1 - j];

                matrix[j][i] = d_l;
                matrix[matrix.size() - 1 - i][j] = d_r;
                matrix[matrix.size() - 1 - j][matrix.size() - 1 - i] = t_r;
                matrix[i][matrix.size() - 1 - j] = t_l;
             }
        }

        if (matrix.size() % 2 != 0) {
            for (int i = 0; i < n; i++) {
                int t_l = matrix[i][n];
                int t_r = matrix[n][matrix.size() - 1 - i];
                int d_r = matrix[matrix.size() - 1 - i][n];
                int d_l = matrix[n][i];

                matrix[i][n] = d_l;
                matrix[n][matrix.size() - 1 - i] = t_l;
                matrix[matrix.size() - 1 - i][n] = t_r;
                matrix[n][i] = d_r;
            }
        }
    }
};
```