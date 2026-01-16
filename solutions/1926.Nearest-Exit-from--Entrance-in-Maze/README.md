# 1926. Nearest Exit from Entrance in Maze

### Description

You are given an m x n matrix maze (0-indexed) with empty cells (represented as '.') and walls (represented as '+'). You are also given the entrance of the maze, where entrance = [entrancerow, entrancecol] denotes the row and column of the cell you are initially standing at.

In one step, you can move one cell up, down, left, or right. You cannot step into a cell with a wall, and you cannot step outside the maze. Your goal is to find the nearest exit from the entrance. An exit is defined as an empty cell that is at the border of the maze. The entrance does not count as an exit.

Return the number of steps in the shortest path from the entrance to the nearest exit, or -1 if no such path exists.

### Example

###### Example I

![](./nearest1-grid.jpg)

> Input: maze = [["+","+",".","+"],[".",".",".","+"],["+","+","+","."]], entrance = [1,2]
> Output: 1
> Explanation: There are 3 exits in this maze at [1,0], [0,2], and [2,3].
> Initially, you are at the entrance cell [1,2].
> - You can reach [1,0] by moving 2 steps left.
> - You can reach [0,2] by moving 1 step up.
> It is impossible to reach [2,3] from the entrance.
> Thus, the nearest exit is [0,2], which is 1 step away.

###### Example II

![](./nearesr2-grid.jpg)

> Input: maze = [["+","+","+"],[".",".","."],["+","+","+"]], entrance = [1,0]
> Output: 2
> Explanation: There is 1 exit in this maze at [1,2].
> [1,0] does not count as an exit since it is the entrance cell.
> Initially, you are at the entrance cell [1,0].
> - You can reach [1,2] by moving 2 steps right.
> Thus, the nearest exit is [1,2], which is 2 steps away.

###### Example III

![](./nearest3-grid.jpg)

> Input: maze = [[".","+"]], entrance = [0,0]
> Output: -1
> Explanation: There are no exits in this maze.

### Solution

广度优先搜索。

```c++
class Solution {
public:
    int nearestExit(vector<vector<char>>& maze, vector<int>& entrance) {
        int m = maze.size();
        int n = maze[0].size();
        vector<int> dx = {1, 0, -1, 0};
        vector<int> dy = {0, 1, 0, -1};
        queue<tuple<int, int, int>> q;
        q.emplace(entrance[0], entrance[1], 0);
        maze[entrance[0]][entrance[1]] = '+';
        while (!q.empty()){
            auto [cx, cy, d] = q.front();
            q.pop();
            for (int k = 0; k < 4; ++k){
                int nx = cx + dx[k];
                int ny = cy + dy[k];
                if (nx >= 0 && nx < m && ny >= 0 && ny < n && maze[nx][ny] == '.'){
                    if (nx == 0 || nx == m - 1 || ny == 0 || ny == n - 1){
                        return d + 1;
                    }
                    maze[nx][ny] = '+';
                    q.emplace(nx, ny, d + 1);
                }
            }
        }
        return -1;
    }
};
```
