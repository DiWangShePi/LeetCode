# 74. Search a 2D Matrix

### 题目描述

给定一个具有以下两个属性的 m x n 整数矩阵：
- 每行按非递减顺序排序。
- 每行的第一个整数大于前一行的最后一个整数。
给定一个整数目标，如果目标在矩阵中，则返回 true，否则返回 false。

您必须在 O(log(m * n)) 时间复杂度内编写解决方案。

```
Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 3
Output: true
```

```
Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 13
Output: false
```

### 题目解析

两次二分，第一次在第一列竖着，第二次在找到的行横着。

### 代码实现

###### c++

```c++
class Solution {
public:
    bool searchMatrix(vector<vector<int>>& matrix, int target) {
        int left = 0, right = matrix.size() - 1, middle = 0;
        while (left <= right) {
            middle = left + (right - left) / 2;
            if (matrix[middle][0] > target) right = middle - 1;
            else if (matrix[middle][0] < target) left = middle + 1; 
            else return true;
        }
        int targetRow = right > -1 ? right : 0;
        left = 0, right = matrix[targetRow].size() - 1;
        while (left <= right) {
            middle = left + (right - left) / 2;
            if (matrix[targetRow][middle] > target) right = middle - 1;
            else if (matrix[targetRow][middle] < target) left = middle + 1;
            else return true;
        }
        return false;
    }
};
```