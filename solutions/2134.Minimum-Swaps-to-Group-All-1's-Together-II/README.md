# 2134. Minimum Swaps to Group All 1's Together II

**Tags:** Sliding Window

### Description

A swap is defined as taking two distinct positions in an array and swapping the values in them.

A circular array is defined as an array where we consider the first element and the last element to be adjacent.

Given a binary circular array nums, return the minimum number of swaps required to group all 1's present in the array together at any location.

### Example

###### Example I

> Input: nums = [0,1,0,1,1,0,0]
> Output: 1
> Explanation: Here are a few of the ways to group all the 1's together:
> [0,0,1,1,1,0,0] using 1 swap.
> [0,1,1,1,0,0,0] using 1 swap.
> [1,1,0,0,0,0,1] using 2 swaps (using the circular property of the array).
> There is no way to group all 1's together with 0 swaps.
> Thus, the minimum number of swaps required is 1.

###### Example II

> Input: nums = [0,1,1,1,0,0,1,1,0]
> Output: 2
> Explanation: Here are a few of the ways to group all the 1's together:
> [1,1,1,0,0,0,0,1,1] using 2 swaps (using the circular property of the array).
> [1,1,1,1,1,0,0,0,0] using 2 swaps.
> There is no way to group all 1's together with 0 or 1 swaps.
> Thus, the minimum number of swaps required is 2.

###### Example III

> Input: nums = [1,1,0,0,1]
> Output: 0
> Explanation: All the 1's are already grouped together due to the circular property of the array.
> Thus, the minimum number of swaps required is 0.

### Solution

先获得值为 1 的元素个数，再用滑动窗口。参考：https://leetcode.cn/discuss/post/3578981/ti-dan-hua-dong-chuang-kou-ding-chang-bu-rzz7/

```c++
class Solution {
public:
    int minSwaps(vector<int>& nums) {
        int count = accumulate(nums.begin(), nums.end(), 0);

        int current = 0, i = 0; 
        for ( ; i < count; i++) current += nums[i];
        int an = count - current;
        for ( ; i < nums.size() + count; i++) {
            current += nums[i % nums.size()];
            current -= nums[i - count];

            an = min(an, count - current);
        }
        return an;
    }
};
```
