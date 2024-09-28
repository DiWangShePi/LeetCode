# 57. Insert Interval

### 题目描述

给定一个不包含重叠区间的区间列表，每个区间包含start和end两个属性，指示区间的范围。列表已按照start进行升序排列。

另外给定一个需要插入的区间，将该区间插入到列表合适的位置中，并保证插入后的列表依然是升序排列且不重叠的。

**示例：**

```
Input: intervals = [[1,3],[6,9]], newInterval = [2,5]
Output: [[1,5],[6,9]]
```

```
Input: intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]], newInterval = [4,8]
Output: [[1,2],[3,10],[12,16]]
Explanation: Because the new interval [4,8] overlaps with [3,5],[6,7],[8,10].
```

### 题目解析

遍历数组，遇到的第一个end大于插入区间start的，即为重叠且可插入的位置。
插入数组后，按照前一题的方式处理剩下的数组。

### 代码实现

###### c++

```c++
class Solution {
public:
    vector<vector<int>> insert(vector<vector<int>>& intervals, vector<int>& newInterval) {
        intervals.push_back(newInterval);

        sort(intervals.begin(), intervals.end(), [](vector<int> &a, vector<int> &b){
            return a[0] < b[0];
        });

        vector<vector<int>> result;
        result.push_back(intervals[0]);

        for (int i = 1; i < intervals.size(); i++) {
            vector<int>& pre = result.back();
            vector<int> cur = intervals[i];

            if (cur[0] <= pre[1]) pre[1] = cur[1] > pre[1] ? cur[1] : pre[1];
            else result.push_back(cur);
        }

        return result;
    }
};
```