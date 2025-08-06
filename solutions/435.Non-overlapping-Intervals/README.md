# 435. Non-overlapping Intervals

### Description

Given an array of intervals intervals where intervals[i] = [starti, endi], return the minimum number of intervals you need to remove to make the rest of the intervals non-overlapping.

Note that intervals which only touch at a point are non-overlapping. For example, [1, 2] and [2, 3] are non-overlapping.

### Example

###### Example I

> Input: intervals = [[1,2],[2,3],[3,4],[1,3]]
> Output: 1
> Explanation: [1,3] can be removed and the rest of the intervals are non-overlapping.

###### Example II

> Input: intervals = [[1,2],[1,2],[1,2]]
> Output: 2
> Explanation: You need to remove two [1,2] to make the rest of the intervals non-overlapping.

###### Example III

> Input: intervals = [[1,2],[2,3]]
> Output: 0
> Explanation: You don't need to remove any of the intervals since they're already non-overlapping.

### Solution

按照区间的结束时间升序排序。

对于遍历排序后的区间：

- 如果当前区间的起始时间 ≥ 上一个区间的结束时间，说明可以保留；

- 否则，有重叠，就必须移除当前区间（或不更新 end）。

最后，用总区间数减去可保留的数量，即为最少移除数量。

```c++
class Solution {
public:
    int eraseOverlapIntervals(vector<vector<int>>& intervals) {
        if (intervals.empty()) return 0;

        sort(intervals.begin(), intervals.end(), [](const vector<int>& a, const vector<int>& b) {
            return a[1] < b[1];
        });

        int count = 1; 
        int end = intervals[0][1]; 

        for (int i = 1; i < intervals.size(); ++i) {
            if (intervals[i][0] >= end) {
                ++count;
                end = intervals[i][1];
            }
        }

        return intervals.size() - count;
    }
};
```
