# 454. 4Sum II

### Description

Given four integer arrays nums1, nums2, nums3, and nums4 all of length n, return the number of tuples (i, j, k, l) such that:

- 0 <= i, j, k, l < n
- nums1[i] + nums2[j] + nums3[k] + nums4[l] == 0

### Example

###### Example I

> Input: nums1 = [1,2], nums2 = [-2,-1], nums3 = [-1,2], nums4 = [0,2]
> Output: 2
> Explanation:
> The two tuples are:
> 1. (0, 0, 0, 1) -> nums1[0] + nums2[0] + nums3[0] + nums4[1] = 1 + (-2) + (-1) + 2 = 0
> 2. (1, 1, 0, 0) -> nums1[1] + nums2[1] + nums3[0] + nums4[0] = 2 + (-1) + (-1) + 0 = 0

###### Example II

> Input: nums1 = [0], nums2 = [0], nums3 = [0], nums4 = [0]
> Output: 1

### Solution

跟Two Sum其实一样，区别在于哪里的一个数组对应这里的两个数组循环遍历，获取所有组合。

```c++
class Solution {
public:
    int fourSumCount(vector<int>& nums1, vector<int>& nums2, vector<int>& nums3, vector<int>& nums4) {
        unordered_map<int, int> ab;
        for (int a : nums1) {
            for (int b : nums2) {
                ab[a + b]++;
            }
        }

        int an = 0;
        for (int c : nums3) {
            for (int d : nums4) {
                if (ab.count(- c - d)) an += ab[- c - d];
            }
        }
        return an;
    }
};
```
