# 539. Minimum Time Difference

**Tags:** String, Math

### Description

Given a list of 24-hour clock time points in "HH:MM" format, return the minimum minutes difference between any two time-points in the list.

### Example

###### Example I

> Input: timePoints = ["23:59","00:00"]
> Output: 1

###### Example II

> Input: timePoints = ["00:00","23:59","00:00"]
> Output: 0

### Solution

所有的时间转为以分钟为单位的数字，装入列表排序后逐个检查。相邻的相减，额外考虑边界情况，即开头的减去末尾的。

```c++
class Solution {
public:
    int findMinDifference(vector<string>& timePoints) {
        vector<int> pq;
        int plus1, hour, minute, t;
        for (const string& time : timePoints) {
            plus1 = time.find(':');
            hour = stoi(time.substr(0, plus1));
            minute = stoi(time.substr(plus1 + 1, time.length() - plus1 - 1));

            t = hour * 60 + minute;
            pq.push_back(t);
        }
        
        sort(pq.begin(), pq.end());
        int an = INT_MAX;
        for (int i = 1; i <= pq.size(); i++) {
            int t = (pq[i % pq.size()] - pq[(i - 1) % pq.size()] + 1440) % 1440;
            an = min(an, t);
        }
        return an;
    }
};
```
