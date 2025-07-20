# 414. Third Maximum Number

### Description

Given an integer array nums, return the third distinct maximum number in this array. If the third maximum does not exist, return the maximum number.

### Example 

###### Example I

> Input: nums = [3,2,1]
> Output: 1
> Explanation:
> The first distinct maximum is 3.
> The second distinct maximum is 2.
> The third distinct maximum is 1.

###### Example II

> Input: nums = [1,2]
> Output: 2
> Explanation:
> The first distinct maximum is 2.
> The second distinct maximum is 1.
> The third distinct maximum does not exist, so the maximum (2) is returned instead.

###### Example III

> Input: nums = [2,2,3,1]
> Output: 1
> Explanation:
> The first distinct maximum is 3.
> The second distinct maximum is 2 (both 2's are counted together since they have the same value).
> The third distinct maximum is 1.

### Solution

简单的做法是用堆存储，用字典确认是否出现

```c++
class Solution {
public:
    int thirdMax(vector<int>& nums) {
        unordered_map<int, bool> exist;
        priority_queue<int> pq;

        for (int num : nums) {
            if (exist.count(num) == 0) pq.push(num);
            exist[num] = true;
        }

        if (pq.size() >= 3) {pq.pop(); pq.pop();}
        return pq.top();
    }
};
```

如果要在空间上优化，我们可以在加入数据的时候将堆的大小限定在3。如果要在时间上优化，我们可以用三个常量来记录答案。
