# 239. Sliding Window Maximum

### Description

You are given an array of integers nums, there is a sliding window of size k which is moving from the very left of the array to the very right. You can only see the k numbers in the window. Each time the sliding window moves right by one position.

Return the max sliding window.

### Solution

暴力的做法是对于每个元素，遍历以当前元素为左端点，长度为k的子序列，找到最大值作为答案填入。

```c++
class Solution {
public:
    vector<int> maxSlidingWindow(vector<int>& nums, int k) {
        vector<int> result;
        for (int i = 0; i < nums.size() - k + 1; i++) {
            int current = nums[i];
            for (int j = 1; j < k; j++) current = max(current, nums[i + j]);
            result.push_back(current);
        }
        return result;
    }
};
```

一个聪明一些的做法是使用优先队列，队列中的元素是Nums值和该值的下标i组成的键值对。
由于我们关心最大值，当当前堆堆顶元素的i值小于等于`i-k`时，即代表该元素不在当前窗口内（而且此后也不会在）。
因此直接删除，满足即为目标值。

虽然这意味着堆内不总是有K个元素，可能有此前较小的值在堆中，但这并不影响结果。因为我们只取堆顶的值。

```c++
class Solution {
public:
    vector<int> maxSlidingWindow(vector<int>& nums, int k) {
        priority_queue<pair<int, int>> q;
        for (int i = 0; i < k; i++) q.emplace(nums[i], i);

        vector<int> an{q.top().first};
        for (int i = k; i < nums.size(); i++) {
            q.emplace(nums[i], i);
            while (q.top().second <= i - k) q.pop();
            an.push_back(q.top().first);
        }
        return an;
    }
};
```

如果想解决这个问题，可以采用单调队列。每一次加入元素前，在队列尾部剔除掉比新加入元素小的元素（因为他们此后永远不会成为答案）。

```c++
class Solution {
public:
    vector<int> maxSlidingWindow(vector<int>& nums, int k) {
        deque<int> q;
        for (int i = 0; i < k; i++) {
            while (!q.empty() && nums[i] >= nums[q.back()]) q.pop_back();
            q.push_back(i);
        }

        vector<int> an{nums[q.front()]};
        for (int i = k; i < nums.size(); i++) {
            while (!q.empty() && nums[i] >= nums[q.back()]) q.pop_back();
            q.push_back(i);

            while (q.front() <= i - k) q.pop_front();
            an.push_back(nums[q.front()]);
        }
        return an;
    }
};
```
