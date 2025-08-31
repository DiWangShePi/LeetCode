# 463. Island Perimeter

### Description

You are given row x col grid representing a map where grid[i][j] = 1 represents land and grid[i][j] = 0 represents water.

Grid cells are connected horizontally/vertically (not diagonally). The grid is completely surrounded by water, and there is exactly one island (i.e., one or more connected land cells).

The island doesn't have "lakes", meaning the water inside isn't connected to the water around the island. One cell is a square with side length 1. The grid is rectangular, width and height don't exceed 100. Determine the perimeter of the island.

### Example 

###### Example I

![](./island.png)

> Input: grid = [[0,1,0,0],[1,1,1,0],[0,1,0,0],[1,1,0,0]]
> Output: 16
> Explanation: The perimeter is the 16 yellow stripes in the image above.

###### Example II

> Input: grid = [[1]]
> Output: 4

###### Example III

> Input: grid = [[1,0]]
> Output: 4

### Solution

遍历，检查每一个岛屿块周边是水还是别的岛屿块

```c++
class Solution {
public:
    int islandPerimeter(vector<vector<int>>& grid) {
        int an = 0;
        int n = grid.size(), m = grid[0].size();
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < m; j++) {
                if (grid[i][j] == 1) {
                    if ((i - 1 > -1 && grid[i - 1][j] == 0) || i - 1 < 0) an++;
                    if ((i + 1 < n && grid[i + 1][j] == 0) || i + 1 >= n) an++;
                    if ((j - 1 > -1 && grid[i][j - 1] == 0) || j - 1 < 0) an++;
                    if ((j + 1 < m && grid[i][j + 1] == 0) || j + 1 >= m) an++;
                }
            }
        }
        return an;
    }
};
```

不用遍历，用DFS来仅获取岛屿块应该会快一点。
