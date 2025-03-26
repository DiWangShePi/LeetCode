# 218. The Skyline Problem

### Description

A city's skyline is the outer contour of the silhouette formed by all the buildings in that city when viewed from a distance. Given the locations and heights of all the buildings, return the skyline formed by these buildings collectively.

The geometric information of each building is given in the array buildings where buildings[i] = [lefti, righti, heighti]:

lefti is the x coordinate of the left edge of the ith building.
righti is the x coordinate of the right edge of the ith building.
heighti is the height of the ith building.
You may assume all buildings are perfect rectangles grounded on an absolutely flat surface at height 0.

The skyline should be represented as a list of "key points" sorted by their x-coordinate in the form [[x1,y1],[x2,y2],...]. Each key point is the left endpoint of some horizontal segment in the skyline except the last point in the list, which always has a y-coordinate 0 and is used to mark the skyline's termination where the rightmost building ends. Any ground between the leftmost and rightmost buildings should be part of the skyline's contour.

Note: There must be no consecutive horizontal lines of equal height in the output skyline. For instance, [...,[2 3],[4 5],[7 5],[11 5],[12 7],...] is not acceptable; the three lines of height 5 should be merged into one in the final output as such: [...,[2 3],[4 5],[12 7],...]

### Solution

解题的核心思路是将建筑物拆解为一系列的「事件点」，然后按 x 坐标排序处理。每个建筑物可以拆分为两个事件：左端点（高度为负值，表示进入）和右端点（高度为正值，表示退出）。排序时，我们需要确保当 x 坐标相同时，先处理左端点再处理右端点，以保证正确的高度变化顺序。接着，我们使用 map<int, int> 维护当前活跃的建筑高度，key 代表高度值，value 代表该高度的计数。当遇到左端点时，将其高度加入 map；当遇到右端点时，从 map 中删除该高度，如果计数为零，则彻底移除。每次处理完一个事件，我们都检查 map 的最大值是否发生变化，若发生变化，则说明 x 处形成了新的天际线拐点，我们将其记录下来。由于 map 是自动有序的，我们可以在 O(log N) 时间内高效维护最高建筑高度。最终，我们按序返回这些关键点，形成完整的天际线轮廓。

### Implementation

###### c++

```c++
class Solution {
public:
    vector<vector<int>> getSkyline(vector<vector<int>>& buildings) {
        vector<pair<int, int>> events;

        for (auto& b : buildings) {
            events.emplace_back(b[0], -b[2]); 
            events.emplace_back(b[1], b[2]); 
        }

        // 排序规则
        //    a) 按 x 坐标升序
        //    b) x 坐标相同情况下：
        //       - 左端点（高度为负数）排在前（保证先加入高度）
        //       - 右端点（高度为正数）排在后（保证先移除高度）
        sort(events.begin(), events.end(), [](pair<int, int> a, pair<int, int> b) {
            return a.first < b.first || (a.first == b.first && a.second < b.second);
        });

        map<int, int> heightMap;
        heightMap[0] = 1;        
        vector<vector<int>> result;
        int prevMax = 0;

        for (auto& e : events) {
            int x = e.first, h = e.second;

            if (h < 0) {
                heightMap[-h]++;
            } else {
                if (--heightMap[h] == 0) {
                    heightMap.erase(h);
                }
            }

            int currMax = heightMap.rbegin()->first;
            if (currMax != prevMax) {
                result.push_back({x, currMax});
                prevMax = currMax;
            }
        }

        return result;
    }
};
```

> using Priority Queue

```c++
class Solution {
public:
    vector<vector<int>> getSkyline(vector<vector<int>>& buildings) {
        vector<pair<int, int>> points;

        for (auto& b : buildings) {
            points.emplace_back(b[0], -b[2]); 
            points.emplace_back(b[1], b[2]); 
        }
        
        sort(points.begin(), points.end(), [](pair<int, int> a, pair<int, int> b) {
            return a.first < b.first || (a.first == b.first && a.second < b.second);
        });

        multiset<int> heights = {0}; 
        vector<vector<int>> result;
        int prevMax = 0;

        for (auto& p : points) {
            if (p.second < 0) {
                heights.insert(-p.second);
            } else {
                heights.erase(heights.find(p.second));
            }

            int currMax = *heights.rbegin();
            if (currMax != prevMax) {
                result.push_back({p.first, currMax});
                prevMax = currMax;
            }
        }

        return result;
    }
};
```
