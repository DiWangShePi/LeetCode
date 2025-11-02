# 554. Brick Wall

### Description

There is a rectangular brick wall in front of you with n rows of bricks. The ith row has some number of bricks each of the same height (i.e., one unit) but they can be of different widths. The total width of each row is the same.

Draw a vertical line from the top to the bottom and cross the least bricks. If your line goes through the edge of a brick, then the brick is not considered as crossed. You cannot draw a line just along one of the two vertical edges of the wall, in which case the line will obviously cross no bricks.

Given the 2D array wall that contains the information about the wall, return the minimum number of crossed bricks after drawing such a vertical line.

### Example

###### Example I

![](./a.png)

> Input: wall = [[1,2,2,1],[3,1,2],[1,3,2],[2,4],[3,1,2],[1,3,1,1]]
> Output: 2

###### Example II

> Input: wall = [[1],[1],[1]]
> Output: 3

### Solution

用字典记录每一层的gap位置，最后统计哪个位置的 gap 最多，墙的厚度减去 gap 就是答案。

```c++
class Solution {
public:
    int leastBricks(vector<vector<int>>& wall) {
        unordered_map<long long, int> gapCount;
        
        for (vector<int>& row : wall) {
            long long sum = 0;
            for (int i = 0; i < row.size() - 1; i++) {
                sum += row[i];
                gapCount[sum]++;
            }
        }
        
        int maxGaps = 0;
        for (auto& [position, count] : gapCount) {
            maxGaps = max(maxGaps, count);
        }
        
        return wall.size() - maxGaps;
    }
};
```
