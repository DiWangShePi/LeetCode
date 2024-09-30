# 59. Spiral Matrix II

### 题目描述

给定正整数n，生成一个n乘n的矩阵，以螺旋的形式在其中填入数字1到$n^2$。

**示例：**

```
Input: n = 3
Output: [[1,2,3],[8,9,4],[7,6,5]]
```

```
Input: n = 1
Output: [[1]]
```

### 题目解析

生成n乘n的空白矩阵，随后按照螺旋的形式在其中填入数字。

### 代码实现

###### c++

```c++
class Solution {
public:
    vector<vector<int>> generateMatrix(int n) {
        vector<vector<int>> result(n, vector<int>(n, 0));
        int i = 0, j = 0;
        int di = 0, dj = 1;

        for (int num = 1; num < (n * n + 1); num++) {
            result[i][j] = num;

            if ((i + di < 0) || (i + di > n -1) || (j + dj < 0) || (j + dj > n-1) || (result[i + di][j + dj] != 0)) {
                int temp = dj;
                dj = -di;
                di = temp;
            }

            i = i + di;
            j = j + dj;
        }
        return result;
    }
};
```