# 3634. Minimum Removals to Balance Array

**Tags:** Sliding Window

### Description

You are given an integer array nums and an integer k.

An array is considered balanced if the value of its maximum element is at most k times the minimum element.

You may remove any number of elements from nums​​​​​​​ without making it empty.

Return the minimum number of elements to remove so that the remaining array is balanced.

Note: An array of size 1 is considered balanced as its maximum and minimum are equal, and the condition always holds true.

### Example

###### Example I

> Input: nums = [2,1,5], k = 2
> Output: 1
> Explanation:
> Remove nums[2] = 5 to get nums = [2, 1].
> Now max = 2, min = 1 and max <= min * k as 2 <= 1 * 2. Thus, the answer is 1.

###### Example II

> Input: nums = [1,6,2,9], k = 3
> Output: 2
> Explanation:
> Remove nums[0] = 1 and nums[3] = 9 to get nums = [6, 2].
> Now max = 6, min = 2 and max <= min * k as 6 <= 2 * 3. Thus, the answer is 2.

###### Example III

> Input: nums = [4,6], k = 2
> Output: 0
> Explanation:
> Since nums is already balanced as 6 <= 4 * 2, no elements need to be removed.

### Solution

将数组排序一遍，然后找最长的符合要求的子数组，总长度减去该子数组的长度即为答案。

找最长的符合要求的子数组可以用滑动窗口解决。

```c++
class Solution {
public:
    int minRemoval(vector<int>& nums, int k) {
        sort(nums.begin(), nums.end());
        int l = 0, r = 1, an = 1;

        while (r < nums.size()) {
            if ((long long)nums[l] * k >= (long long)nums[r]) r++;
            else l++;

            an = max(an, r - l);
        }
        return nums.size() - max(an, r - l);
    }
};
```
