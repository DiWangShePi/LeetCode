# 219. Contains Duplicate II

### Description

Given an integer array nums and an integer k, return true if there are two distinct indices i and j in the array such that nums[i] == nums[j] and abs(i - j) <= k.

### Solution

同样是遍历一遍，如果遇到重复的值，检查当前值和前值对应的i之差是否满足条件。
满足即返回true，不满足则将当前按值对应的i更新为现在的i。

### Implementation

###### c++

```c++
class Solution {
public:
    bool containsNearbyDuplicate(vector<int>& nums, int k) {
        unordered_map<int, int> dict;
        for (int i = 0; i < nums.size(); i++) {
            int val = nums[i];
            if (dict.count(val) == 0) dict[val] = i;
            else {
                // cout << val << " " << i << " " << dict[i] << endl;
                if (i - dict[val] <= k) return true;
                else dict[val] = i;
            }
        }
        return false;
    }
};
```
