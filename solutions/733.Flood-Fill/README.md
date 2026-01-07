# 733. Flood Fill

**Tags:** DFS

### Description

You are given an image represented by an m x n grid of integers image, where image[i][j] represents the pixel value of the image. You are also given three integers sr, sc, and color. Your task is to perform a flood fill on the image starting from the pixel image[sr][sc].

To perform a flood fill:

1. Begin with the starting pixel and change its color to color.
2. Perform the same process for each pixel that is directly adjacent (pixels that share a side with the original pixel, either horizontally or vertically) and shares the same color as the starting pixel.
3. Keep repeating this process by checking neighboring pixels of the updated pixels and modifying their color if it matches the original color of the starting pixel.
4. The process stops when there are no more adjacent pixels of the original color to update.

Return the modified image after performing the flood fill.

### Example

###### Example I

> Input: image = [[1,1,1],[1,1,0],[1,0,1]], sr = 1, sc = 1, color = 2
> Output: [[2,2,2],[2,2,0],[2,0,1]]
> Explanation:
> ![](./flood1-grid.jpg) 
> 
> From the center of the image with position (sr, sc) = (1, 1) (i.e., the red pixel), all pixels connected by a path of the > same color as the starting pixel (i.e., the blue pixels) are colored with the new color.
> Note the bottom corner is not colored 2, because it is not horizontally or vertically connected to the starting pixel.

###### Example II

> Input: image = [[0,0,0],[0,0,0]], sr = 0, sc = 0, color = 0
> Output: [[0,0,0],[0,0,0]]
> Explanation:
> The starting pixel is already colored with 0, which is the same as the target color. Therefore, no changes are made to the image.

### Solution

从给定的节点开始深度优先遍历。

```c++
class Solution {
public:
    vector<vector<int>> floodFill(vector<vector<int>>& image, int sr, int sc, int color) {
        int m = image.size(), n = image[0].size();
        vector<vector<bool>> visited(m, vector<bool>(n, false));
        dfs(image, visited, sr, sc, color, image[sr][sc]);
        return image;
    }

private:
    void dfs(vector<vector<int>>& image, vector<vector<bool>>& visited, int i, int j, int color, int origin) {
        int m = image.size(), n = image[0].size();
        if (i < 0 || i >= m || j < 0 || j >= n || visited[i][j]) return;

        visited[i][j] = true;
        if (image[i][j] != origin) return;

        image[i][j] = color;
        dfs(image, visited, i + 1, j, color, origin);
        dfs(image, visited, i - 1, j, color, origin);
        dfs(image, visited, i, j + 1, color, origin);
        dfs(image, visited, i, j - 1, color, origin);
        return;
    }
};
```
