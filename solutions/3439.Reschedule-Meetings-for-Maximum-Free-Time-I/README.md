# 3439. Reschedule Meetings for Maximum Free Time I

**Tags:** Sliding Window

### Description

You are given an integer eventTime denoting the duration of an event, where the event occurs from time t = 0 to time t = eventTime.

You are also given two integer arrays startTime and endTime, each of length n. These represent the start and end time of n non-overlapping meetings, where the ith meeting occurs during the time [startTime[i], endTime[i]].

You can reschedule at most k meetings by moving their start time while maintaining the same duration, to maximize the longest continuous period of free time during the event.

The relative order of all the meetings should stay the same and they should remain non-overlapping.

Return the maximum amount of free time possible after rearranging the meetings.

Note that the meetings can not be rescheduled to a time outside the event.

### Example

###### Example I

> Input: eventTime = 5, k = 1, startTime = [1,3], endTime = [2,5]
> Output: 2

![](./example0_rescheduled.png)

###### Example II

> Input: eventTime = 10, k = 1, startTime = [0,2,9], endTime = [1,4,10]
> Output: 6

![](./example1_rescheduled.png)

###### Example III

> Input: eventTime = 5, k = 2, startTime = [0,1,2,3,4], endTime = [1,2,3,4,5]
> Output: 0

### Solution

对于 k 个可以修改的会议，他们可以影响到的会议间隔有 k + 1 个。
遍历给定的数组获取所有初始的间隔，选择其中连续的 k + 1 个，使得总和最大。

```c++
class Solution {
public:
    int maxFreeTime(int eventTime, int k, vector<int>& startTime, vector<int>& endTime) {
        vector<int> intervals;
        if (startTime[0] > 0) intervals.push_back(startTime[0] - 0);
        for (int i = 1; i < startTime.size(); i++) {
            intervals.push_back(startTime[i] - endTime[i - 1]);
        }
        if (endTime.back() < eventTime) intervals.push_back(eventTime - endTime.back());

        int current = 0, an = 0;
        for (int i = 0; i < intervals.size(); i++) {
            current += intervals[i];
            if (i > k)  current -= intervals[i - k - 1];

            an = max(an, current);
        }
        return an;
    }
};
```
