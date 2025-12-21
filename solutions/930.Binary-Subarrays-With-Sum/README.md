# 930. Binary Subarrays With Sum

**Tags:** Prefix Sum, Sliding Window

### Description

Given a binary array nums and an integer goal, return the number of non-empty subarrays with a sum goal.

A subarray is a contiguous part of the array.

### Example

###### Example I

> Input: nums = [1,0,1,0,1], goal = 2
> Output: 4
> Explanation: The 4 subarrays are bolded and underlined below:
> [1,0,1,0,1]
> [1,0,1,0,1]
> [1,0,1,0,1]
> [1,0,1,0,1]

###### Example II

> Input: nums = [0,0,0,0,0], goal = 0
> Output: 15

### Solution

用前缀和可以解决这个问题，累加的过程中，记录当前的和，再记录满足条件的（sum - goal）的子数组有多少个，即可。

```c++
class Solution {
public:
    int numSubarraysWithSum(vector<int>& nums, int goal) {
        unordered_map<int, int> sums;
        int sum = 0, n = nums.size(), an = 0;
        for (int i = 0; i < n; i++) {
            sums[sum]++;
            sum += nums[i];
            an += sums[sum - goal];
        }
        return an;
    }
};
```

注意这里我们是先记录了之前的和，再更新现在的和并查找新值。

另一个有趣的做法基于这一想法：元素和恰好为 K 的子数组个数，应该等于元素和小于等于 K 的子数组个数，减去元素和小于K（小于等于 K - 1）的子数组个数。

至于如何计算元素和小于等于 K 的子数组个数，采用的方法是滑动窗口。
对于每一个满足条件的子数组 [l, r - 1]，如果 [l, r] 也同样满足条件，那么加入 r 增加的子数组数量，应该是 r - l + 1 个。
即每一个以 r 为右端点的子数组（连续的）。

由此，我们可以使用滑动数组，在右边界更新的时候拓展答案，移动左边界以满足条件。

```c++
class Solution {
public:
    int numSubarraysWithSum(vector<int>& nums, int goal) {
        return sumE(nums, goal) - sumE(nums, goal - 1);
    }

private:
    int sumE(vector<int>& nums, int goal) {
        if (goal < 0) return 0;
        int an = 0, sum = 0, l = 0;
        for (int i = 0; i < nums.size(); i++) {
            sum += nums[i];
            while (sum > goal) sum -= nums[l++];
            an += i - l + 1;
        }
        return an;
    }
};
```
