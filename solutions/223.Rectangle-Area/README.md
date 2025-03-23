# 223. Rectangle Area

### Description

Given the coordinates of two rectilinear rectangles in a 2D plane, return the total area covered by the two rectangles.

The first rectangle is defined by its bottom-left corner (ax1, ay1) and its top-right corner (ax2, ay2).

The second rectangle is defined by its bottom-left corner (bx1, by1) and its top-right corner (bx2, by2).

### Solution

用两个长方形的面积之和减去重叠部分的面积。

### Implementation

###### c++

```c++
class Solution {
public:
    int computeArea(int ax1, int ay1, int ax2, int ay2, int bx1, int by1, int bx2, int by2) {
        int area1 = (ax2 - ax1) * (ay2 - ay1), area2 = (bx2 - bx1) * (by2 - by1);
        int overlapWidth = min(ax2, bx2) - max(ax1, bx1), overlapHeight = min(ay2, by2) - max(ay1, by1);
        int overlapArea = max(overlapWidth, 0) * max(overlapHeight, 0);
        return area1 + area2 - overlapArea;
    }
};
```
