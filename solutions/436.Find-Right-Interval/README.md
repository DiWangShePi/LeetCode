# 436. Find Right Interval

### Description

You are given an array of intervals, where intervals[i] = [starti, endi] and each starti is unique.

The right interval for an interval i is an interval j such that startj >= endi and startj is minimized. Note that i may equal j.

Return an array of right interval indices for each interval i. If no right interval exists for interval i, then put -1 at index i.

### Example

###### Example I

> Input: intervals = [[1,2]]
> Output: [-1]
> Explanation: There is only one interval in the collection, so it outputs -1.

###### Example II

> Input: intervals = [[3,4],[2,3],[1,2]]
> Output: [-1,0,1]
> Explanation: There is no right interval for [3,4].
> The right interval for [2,3] is [3,4] since start0 = 3 is the smallest start that is >= end1 = 3.
> The right interval for [1,2] is [2,3] since start1 = 2 is the smallest start that is >= end2 = 2.

###### Example III

> Input: intervals = [[1,4],[2,3],[3,4]]
> Output: [-1,2,-1]
> Explanation: There is no right interval for [1,4] and [3,4].
> The right interval for [2,3] is [3,4] since start2 = 3 is the smallest start that is >= end1 = 3.

### Solution

我们先考虑暴力的做法：遍历数组中每一个区间，对于每一个区间，再次遍历数组，查找满足条件的区间，并记录最小的哪一个

```c++
class Solution {
public:
    vector<int> findRightInterval(vector<vector<int>>& intervals) {
        int n = intervals.size();

        vector<int> an;
        for (int i = 0; i < n; i++) {
            int end_i = intervals[i][1];
            int start_j = INT_MAX;
            int result = -1;

            for (int j = 0; j < n; j++) {
                if (intervals[j][0] >= end_i) {
                    if (start_j > intervals[j][0]) {
                        result = j;
                        start_j = intervals[j][0];
                    }
                }
            }
            an.push_back(result);
        }
        return an;
    }
};
```

接下来我们考虑优化一些的思路，根据区间的第一个元素进行排序，这样当我们检索到指定元素时，我们知道该元素对应的区间只可能在当前元素的后面，因此就不需要完整的遍历一遍了。

```c++
class Solution {
public:
    vector<int> findRightInterval(vector<vector<int>>& intervals) {
        int n = intervals.size();

        // Create a mapping from intervals to index
        unordered_map<int, int> dict;
        for (int i = 0; i < n; i++) dict[intervals[i][0]] = i;
        
        // sort the intervals based on first element
        sort(intervals.begin(), intervals.end(), [](const vector<int>& a, const vector<int>& b) {
            return a[0] < b[0];
        });

        vector<int> an(n, -1);
        for (int i = 0; i < n; i++) {
            int end_i = intervals[i][1];
            for (int j = i; j < n; j++) {
                if (intervals[j][0] >= end_i) {
                    int index = dict[intervals[i][0]];
                    an[index] = dict[intervals[j][0]];
                    break;
                }
            }
        }
        return an;
    }
};
```

但这个解法在极端情况下，复杂度与之前的是一致的，因为我们依然采用的是遍历的方式去获取目标值。但对于这个有序数组，我们可以用二分查找的方式。

```c++
class Solution {
public:
    vector<int> findRightInterval(vector<vector<int>>& intervals) {
        vector<pair<int, int>> startIntervals;
        int n = intervals.size();
        for (int i = 0; i < n; i++) {
            startIntervals.emplace_back(intervals[i][0], i);
        }
        sort(startIntervals.begin(), startIntervals.end());

        vector<int> ans(n, -1);
        for (int i = 0; i < n; i++) {
            auto it = lower_bound(startIntervals.begin(), startIntervals.end(), make_pair(intervals[i][1], 0));
            if (it != startIntervals.end()) {
                ans[i] = it->second;
            }
        }
        return ans;
    }
};
```
