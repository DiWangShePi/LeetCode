# 498. Diagonal Traverse

### Description

Given an m x n matrix mat, return an array of all the elements of the array in a diagonal order.

### Example 

###### Example I

![](./diag1-grid.jpg)

> Input: mat = [[1,2,3],[4,5,6],[7,8,9]]
> Output: [1,2,4,7,5,3,6,8,9]

###### Example II

> Input: mat = [[1,2],[3,4]]
> Output: [1,2,3,4]

### Solution

遍历一遍，处理好四种边界情况

```c++
class Solution {
public:
    vector<int> findDiagonalOrder(vector<vector<int>>& mat) {
        int n = mat.size(), m = mat[0].size();
        vector<int> an;
        bool towards = true;
        int i = 0, j = 0;
        while (an.size() < n * m) {
            an.push_back(mat[i][j]);

            if (towards) {
                if (i - 1 > -1 && j + 1 < m) {
                    i = i - 1;
                    j = j + 1;
                } else {
                    if (j + 1 < m) j = j + 1;
                    else i = i + 1;
                    towards = false;
                }
            } else {
                if (i + 1 < n && j - 1 > -1) {
                    i = i + 1;
                    j = j - 1;
                } else {
                    if (i + 1 < n) i = i + 1;
                    else j = j + 1;
                    towards = true;
                }
            }
        }
        return an;
    }
};
```
