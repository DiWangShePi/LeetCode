# 1695. Maximum Erasure Value

**Tags:** Sliding Window, Dict

### Description

You are given an array of positive integers nums and want to erase a subarray containing unique elements. The score you get by erasing the subarray is equal to the sum of its elements.

Return the maximum score you can get by erasing exactly one subarray.

An array b is called to be a subarray of a if it forms a contiguous subsequence of a, that is, if it is equal to a[l],a[l+1],...,a[r] for some (l,r).

### Example

###### Example I

> Input: nums = [4,2,4,5,6]
> Output: 17
> Explanation: The optimal subarray here is [2,4,5,6].

###### Example II

> Input: nums = [5,2,1,2,5,2,1,2,5]
> Output: 8
> Explanation: The optimal subarray here is [5,2,1] or [1,2,5].

### Solution

用字典记录出现的数字的个数，字典的大小需要和序列的长度一致（unique的元素），不满足时移动左边界至满足。纪律满足条件的最长子序列的数字和。

```c++
class Solution {
public:
    int maximumUniqueSubarray(vector<int>& nums) {
        int l = 0, an = 0, sum = 0, n = nums.size();
        unordered_map<int, int> dict;
        for (int i = 0; i < n; i++) {
            int num = nums[i];
            if (dict.count(num) == 0) dict[num] = 0;
            dict[num]++;
            sum += num;
            
            while (dict.size() != i - l + 1) {
                dict[nums[l]]--;
                if (dict[nums[l]] == 0) dict.erase(nums[l]);

                sum -= nums[l];
                l++;
            }
            an = max(an, sum);
        }
        return max(an, sum);
    }
};
```
