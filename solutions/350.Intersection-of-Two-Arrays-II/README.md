# 350. Intersection of Two Arrays II

### Description

Given two integer arrays nums1 and nums2, return an array of their intersection. Each element in the result must appear as many times as it shows in both arrays and you may return the result in any order.

### Example 

###### Example I

```
Input: nums1 = [1,2,2,1], nums2 = [2,2]
Output: [2,2]
```

###### Example II

```
Input: nums1 = [4,9,5], nums2 = [9,4,9,8,4]
Output: [4,9]
Explanation: [9,4] is also accepted.
```

### Solution

跟上一题一样。

```c++
class Solution {
public:
    vector<int> intersect(vector<int>& nums1, vector<int>& nums2) {
        vector<int> dict(10001, 0);
        for (int val : nums1) dict[val]++;

        vector<int> an;
        for (int val : nums2) {
            if (dict[val] > 0) {
                an.push_back(val);
                dict[val]--;
            }
        }
        return an;
    }
};
```
