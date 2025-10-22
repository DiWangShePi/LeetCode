# 532. K-diff Pairs in an Array

### Description

Given an array of integers nums and an integer k, return the number of unique k-diff pairs in the array.

A k-diff pair is an integer pair (nums[i], nums[j]), where the following are true:

- 0 <= i, j < nums.length
- i != j
- |nums[i] - nums[j]| == k
Notice that |val| denotes the absolute value of val.

### Example 

###### Example I

> Input: nums = [3,1,4,1,5], k = 2
> Output: 2
> Explanation: There are two 2-diff pairs in the array, (1, 3) and (3, 5).
> Although we have two 1s in the input, we should only return the number of unique pairs.

###### Example II

> Input: nums = [1,2,3,4,5], k = 1
> Output: 4
> Explanation: There are four 1-diff pairs in the array, (1, 2), (2, 3), (3, 4) and (4, 5).

###### Example III

> Input: nums = [1,3,1,5,4], k = 0
> Output: 1
> Explanation: There is one 0-diff pair in the array, (1, 1).

### Solution

我一开始的想法（暴力）可以被优化为双指针的实现方式。
先将数组排序，然后一个指针先走，一个指针后走。数字之差小于目标时，第一个指针继续往后走，数字之差大于目标时，第二个指针继续往后走。

为了确保不记录重复的数字对，相等的时候两个指针都要走，且要将重复的数字走完。

```c++
class Solution {
public:
    int findPairs(vector<int>& nums, int k) {
        sort(nums.begin(), nums.end());

        int left = 0, right = 1, an = 0;
        while (right < nums.size()) {
            if (nums[right] - nums[left] < k) right++;
            else if (nums[right] - nums[left] > k) left++;
            else {
                if (left != right) an++;

                right++;
                while (right < nums.size() && nums[right] == nums[right - 1]) right++;
                left++;
                while (left < nums.size() && nums[left] == nums[left - 1]) left++;

                if (left == right) right++;
            }
        }
        return an;
    }
};
```

一个更优雅的解法是用字典。遍历一遍记录所有数字，然后检查每个数字大于k的数字是否存在。

> 虽然跑起来慢了点，但我觉得挺优雅的

```c++
class Solution {
public:
    int findPairs(vector<int>& nums, int k) {
        unordered_map<int, int> dict;
        for (int num : nums) dict[num]++;

        int an = 0;
        if (k == 0) {
            for (auto& [num, cnt] : dict) {
                if (cnt > 1) an++;
            }
        } else {
            for (auto& [num, cnt] : dict) {
                if (dict.count(num + k)) an++;
            }
        }
        return an;
    }
};
```
