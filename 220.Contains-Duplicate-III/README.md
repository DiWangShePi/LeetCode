# 220. Contains Duplicate III

### Description

You are given an integer array nums and two integers indexDiff and valueDiff.

Find a pair of indices (i, j) such that:

- i != j,
- abs(i - j) <= indexDiff.
- abs(nums[i] - nums[j]) <= valueDiff, and

Return true if such pair exists or false otherwise.

### Solution

首选暴力，对于每一个 i，遍历(i - indexDiff, i + indexDiff)范围内的所有值，检查他们是否满足条件，满足返回 true。

不出所料，这样会超时。

进一步考虑到，题目并不要求我们返回具体的下标和数值。那么，如果我们知道一个 indexDiff 范围内数字的最小值和最大值，就可以检查
这个值的范围是否满足条件。满足即可视为同样满足题目的条件。

于是我们使用桶排序。

### Implementation

###### c++

```c++
class Solution {
public:
    bool containsNearbyAlmostDuplicate(vector<int>& nums, int indexDiff, int valueDiff) {
        unordered_map<int, int> buckets;
        int width = valueDiff + 1;
        for (int i = 0; i < nums.size(); i++) {
            int id = getBucketID(nums[i], width);

            if (buckets.count(id) != 0) return true;
            if (buckets.count(id - 1) && abs(buckets[id - 1] - nums[i]) <= valueDiff) return true;
            if (buckets.count(id + 1) && abs(buckets[id + 1] - nums[i]) <= valueDiff) return true;

            buckets[id] = nums[i];
            if (i >= indexDiff) {
                id = getBucketID(nums[i - indexDiff], width);
                buckets.erase(id);
            }
        }
        return false;
    }

private:
    int getBucketID(int num, int width) {
        return num < 0 ? (num + 1) / width - 1 : num / width;
    }
};
```