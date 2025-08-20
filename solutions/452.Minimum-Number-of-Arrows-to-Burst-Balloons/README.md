# 452. Minimum Number of Arrows to Burst Balloons

### Description

There are some spherical balloons taped onto a flat wall that represents the XY-plane. The balloons are represented as a 2D integer array points where points[i] = [xstart, xend] denotes a balloon whose horizontal diameter stretches between xstart and xend. You do not know the exact y-coordinates of the balloons.

Arrows can be shot up directly vertically (in the positive y-direction) from different points along the x-axis. A balloon with xstart and xend is burst by an arrow shot at x if xstart <= x <= xend. There is no limit to the number of arrows that can be shot. A shot arrow keeps traveling up infinitely, bursting any balloons in its path.

Given the array points, return the minimum number of arrows that must be shot to burst all balloons.

### Example

###### Example I

> Input: points = [[10,16],[2,8],[1,6],[7,12]]
> Output: 2
> Explanation: The balloons can be burst by 2 arrows:
> - Shoot an arrow at x = 6, bursting the balloons [2,8] and [1,6].
> - Shoot an arrow at x = 11, bursting the balloons [10,16] and [7,12].

###### Example II

> Input: points = [[1,2],[3,4],[5,6],[7,8]]
> Output: 4
> Explanation: One arrow needs to be shot for each balloon for a total of 4 arrows.

###### Example III

> Input: points = [[1,2],[2,3],[3,4],[4,5]]
> Output: 2
> Explanation: The balloons can be burst by 2 arrows:
> - Shoot an arrow at x = 2, bursting the balloons [1,2] and [2,3].
> - Shoot an arrow at x = 4, bursting the balloons [3,4] and [4,5].

### Solution

将存在重叠区间的子区间合并，取交叠的那一部分。但如果我们采用现有的方式，复杂度会很高，所以需要先排序一遍。

```c++
class Solution {
public:
    int findMinArrowShots(vector<vector<int>>& points) {
        sort(points.begin(), points.end(), [](const vector<int>& a, const vector<int>& b){
            return a[1] < b[1];
        });

        int an = points.size();
        for (int i = 1; i < points.size(); i++) {
            if (points[i][0] <= points[i - 1][1]) {
                an--;

                points[i][0] = max(points[i][0], points[i - 1][0]);
                points[i][1] = min(points[i][1], points[i - 1][1]);
            }
        }
        return an;
    }
};
```
