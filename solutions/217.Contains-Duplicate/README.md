# 217. Contains Duplicate

### Description

Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.

### Solution

论字典的基本应用

### Implementation

###### c++

```c++
class Solution {
public:
    bool containsDuplicate(vector<int>& nums) {
        unordered_map<int, int> dict;
        for (int val : nums) {
            if (dict.count(val) == 0) dict[val] = 1;
            else return true;
        }
        return false;
    }
};
```
