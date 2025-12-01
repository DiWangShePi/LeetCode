# 1493. Longest Subarray of 1's After Deleting One Element

### Description

Given a binary array nums, you should delete one element from it.

Return the size of the longest non-empty subarray containing only 1's in the resulting array. Return 0 if there is no such subarray.

### Example

###### Example I

> Input: nums = [1,1,0,1]
> Output: 3
> Explanation: After deleting the number in position 2, [1,1,1] contains 3 numbers with value of 1's.

###### Example II

> Input: nums = [0,1,1,1,0,1,1,0,1]
> Output: 5
> Explanation: After deleting the number in position 4, [0,1,1,1,1,1,0,1] longest subarray with value of 1's is [1,1,1,1,1].

###### Example III

> Input: nums = [1,1,1]
> Output: 2
> Explanation: You must delete one element.

### Solution

记录由 0 作为边界构成的子序列的长度，取相邻的两个计算结果，记录最大值。

```c++
class Solution {
public:
    int longestSubarray(vector<int>& nums) {
        vector<int> interval;
        int last = -1, n = nums.size();
        for (int i = 0; i < n; i++) {
            if (nums[i] == 0) {
                interval.push_back(i - last - 1);
                last = i;
            }
        }
        interval.push_back(n - last - 1);

        if (interval.size() == 1) return interval[0] - 1;
        else {
            int an = INT_MIN;
            for (int i = 0; i < interval.size() - 1; i++) {
                an = max(an, interval[i] + interval[i + 1]);
            }
            return an;
        }
    }
};
```
