# 300. Longest Increasing Subsequence

### Description

Given an integer array nums, return the length of the longest strictly increasing subsequence.

### Example 

###### Example I

```
Input: nums = [10,9,2,5,3,7,101,18]
Output: 4
Explanation: The longest increasing subsequence is [2,3,7,101], therefore the length is 4.
```

###### Example II

```
Input: nums = [0,1,0,3,2,3]
Output: 4
```

###### Example III

```
Input: nums = [7,7,7,7,7,7,7]
Output: 1
```

### Solution

我们先考虑暴力的方式：对于每一个[0, i]构成的字序列，遍历每一个元素。
若nums[i] > nums[j]，则i所能构成的最长子序列即为j所能构成的最长子序列数字加1。若不大于，即不可行。

遍历数组，即可获得所有元素作为结尾的最长子字符串的长度。

```c++
class Solution {
public:
    int lengthOfLIS(vector<int>& nums) {
        vector<int> dp;
        int result = 0;
        for (int i = 0; i < nums.size(); i++) {
            dp.push_back(1);
            for (int j = 0; j < i; j++) {
                if (nums[j] < nums[i]) dp[i] = max(dp[i], dp[j] + 1);
            }
            result = max(result, dp[i]);
        }
        return result;
    }
};
```

一个更优雅的解法是采用动态规划和二分查找：
令dp[i]代表以i为长度的递增子序列的末尾值，遇到新元素num时，在dp中查找（二分）满足num < dp[i]的最小的i：
- 若不存在，则将新元素增加至末尾。
- 若存在，则将该值替换掉。
最后返回dp的长度。

```c++
class Solution {
public:
    int lengthOfLIS(vector<int>& nums) {
        vector<int> dp;
        for (int num : nums) {
            auto it = lower_bound(dp.begin(), dp.end(), num);
            if (it == dp.end()) dp.push_back(num);
            else *it = num;
        }
        return dp.size();
    }
};
```