# 56. Merge Intervals

### 题目描述

给定一个列表的区间，每个区间有开始和结束两个值，合并所有有重叠的区间，返回合并结束后的区间列表。

**示例：**

```
Input: intervals = [[1,3],[2,6],[8,10],[15,18]]
Output: [[1,6],[8,10],[15,18]]
Explanation: Since intervals [1,3] and [2,6] overlap, merge them into [1,6].
```

```
Input: intervals = [[1,4],[4,5]]
Output: [[1,5]]
Explanation: Intervals [1,4] and [4,5] are considered overlapping.
```

### 题目解析

首先将所有的元素按开始值进行排序，初始化结果列表为空值，将第一个元素放入列表中。

遍历剩余的数组，每一次比较当前元素是否和结果列表的最后一个值存在重叠的部分。
若存在，则更新数组的最后一个元素。若不存在，将当前值放入到结果列表中。

### 代码实现

###### c++

```c++
class Solution {
public:
    vector<vector<int>> merge(vector<vector<int>>& intervals) {
        sort(intervals.begin(), intervals.end(), [](vector<int> &a, vector<int> &b){
            return a[0] < b[0];
        });

        vector<vector<int>> result;
        result.push_back(intervals[0]);

        for (int i = 1; i < intervals.size(); i++) {
            vector<int> pre = result[result.size() - 1];
            vector<int> cur = intervals[i];

            if (cur[0] <= pre[1]) pre[1] = cur[1] > pre[1] ? cur[1] : pre[1];
            else result.push_back(cur);
        }

        return result;
    }
};
```