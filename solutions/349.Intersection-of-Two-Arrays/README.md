# 349. Intersection of Two Arrays

### Description

Given two integer arrays nums1 and nums2, return an array of their intersection. Each element in the result must be unique and you may return the result in any order.

### Example 

###### Example I

```
Input: nums1 = [1,2,2,1], nums2 = [2,2]
Output: [2]
```

###### Example II

```
Input: nums1 = [4,9,5], nums2 = [9,4,9,8,4]
Output: [9,4]
Explanation: [4,9] is also accepted.
```

### Solution

先遍历一遍`nums1`，再遍历一遍`nums2`。

```c++
class Solution {
public:
    vector<int> intersection(vector<int>& nums1, vector<int>& nums2) {
        vector<int> dict(10001, 0);
        for (int val : nums1) dict[val] = 1;

        vector<int> an;
        for (int val : nums2) {
            if (dict[val] == 1) {
                an.push_back(val);
                dict[val] = 0;
            }
        }
        return an;
    }
};
```
