# 417. Pacific Atlantic Water Flow

### Description

There is an m x n rectangular island that borders both the Pacific Ocean and Atlantic Ocean. The Pacific Ocean touches the island's left and top edges, and the Atlantic Ocean touches the island's right and bottom edges.

The island is partitioned into a grid of square cells. You are given an m x n integer matrix heights where heights[r][c] represents the height above sea level of the cell at coordinate (r, c).

The island receives a lot of rain, and the rain water can flow to neighboring cells directly north, south, east, and west if the neighboring cell's height is less than or equal to the current cell's height. Water can flow from any cell adjacent to an ocean into the ocean.

Return a 2D list of grid coordinates result where result[i] = [ri, ci] denotes that rain water can flow from cell (ri, ci) to both the Pacific and Atlantic oceans.

### Example

###### Example I

![](./waterflow-grid.jpg)

> Input: heights = [[1,2,2,3,5],[3,2,3,4,4],[2,4,5,3,1],[6,7,1,4,5],[5,1,1,2,4]]
> Output: [[0,4],[1,3],[1,4],[2,2],[3,0],[3,1],[4,0]]
> Explanation: The following cells can flow to the Pacific and Atlantic oceans, as shown below:
> [0,4]: [0,4] -> Pacific Ocean 
>        [0,4] -> Atlantic Ocean
> [1,3]: [1,3] -> [0,3] -> Pacific Ocean 
>        [1,3] -> [1,4] -> Atlantic Ocean
> [1,4]: [1,4] -> [1,3] -> [0,3] -> Pacific Ocean 
>        [1,4] -> Atlantic Ocean
> [2,2]: [2,2] -> [1,2] -> [0,2] -> Pacific Ocean 
>        [2,2] -> [2,3] -> [2,4] -> Atlantic Ocean
> [3,0]: [3,0] -> Pacific Ocean 
>        [3,0] -> [4,0] -> Atlantic Ocean
> [3,1]: [3,1] -> [3,0] -> Pacific Ocean 
>        [3,1] -> [4,1] -> Atlantic Ocean
> [4,0]: [4,0] -> Pacific Ocean 
>        [4,0] -> Atlantic Ocean
> Note that there are other possible paths for these cells to flow to the Pacific and Atlantic oceans.

###### Example II

> Input: heights = [[1]]
> Output: [[0,0]]
> Explanation: The water can flow from the only cell to the Pacific and Atlantic oceans.

### Solution

一开始的想法是动态规划，但随后发现dp[i][j]取决于周围四个格子的状态，但此时周围四个可能还不完全知道。

于是改用搜索算法，深度优先搜索和广度优先搜索都是可以的。但对每个格子都进行一次搜索就开销太大了，可以采取记忆化的方式。

```c++
static const int dirs[4][2] = {{-1, 0}, {1, 0}, {0, -1}, {0, 1}};

class Solution {
public:
    vector<vector<int>> heights;

    void dfs(int row, int col, vector<vector<bool>> & ocean) {
        int m = ocean.size();
        int n = ocean[0].size();
        if (ocean[row][col]) {
            return;
        }
        ocean[row][col] = true;
        for (int i = 0; i < 4; i++) {
            int newRow = row + dirs[i][0], newCol = col + dirs[i][1];
            if (newRow >= 0 && newRow < m && newCol >= 0 && newCol < n && heights[newRow][newCol] >= heights[row][col]) {
                dfs(newRow, newCol, ocean);
            }
        }
    }

    vector<vector<int>> pacificAtlantic(vector<vector<int>>& heights) {
        this->heights = heights;
        int m = heights.size();
        int n = heights[0].size();
        vector<vector<bool>> pacific(m, vector<bool>(n, false));
        vector<vector<bool>> atlantic(m, vector<bool>(n, false));

        for (int i = 0; i < m; i++) {
            dfs(i, 0, pacific);
        }
        for (int j = 1; j < n; j++) {
            dfs(0, j, pacific);
        }
        for (int i = 0; i < m; i++) {
            dfs(i, n - 1, atlantic);
        }
        for (int j = 0; j < n - 1; j++) {
            dfs(m - 1, j, atlantic);
        }
        vector<vector<int>> result;
        for (int i = 0; i < m; i++) {
            for (int j = 0; j < n; j++) {
                if (pacific[i][j] && atlantic[i][j]) {
                    vector<int> cell;
                    cell.emplace_back(i);
                    cell.emplace_back(j);
                    result.emplace_back(cell);
                }
            }
        }
        return result;
    }
};
```
