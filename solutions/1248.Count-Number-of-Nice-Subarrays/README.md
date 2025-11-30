# 1248. Count Number of Nice Subarrays

**Tags:** Slinding Window

### Description

Given an array of integers nums and an integer k. A continuous subarray is called nice if there are k odd numbers on it.

Return the number of nice sub-arrays.

### Example

###### Example I

> Input: nums = [1,1,2,1,1], k = 3
> Output: 2
> Explanation: The only sub-arrays with 3 odd numbers are [1,1,2,1] and [1,2,1,1].

###### Example II

> Input: nums = [2,4,6], k = 1
> Output: 0
> Explanation: There are no odd numbers in the array.

###### Example III

> Input: nums = [2,2,2,1,2,2,1,2,2,2], k = 2
> Output: 16

### Solution

可以看作 0，1 数组，偶数为0，奇数为1。随后问题变为有多少子数组的和为 K。

计算有多少子数组的和小于等于 K，再计算有多少子数组的和小于等于 K - 1。这一过程可以用滑动窗口解决。

```c++
class Solution {
public:
    int numberOfSubarrays(vector<int>& nums, int k) {
        return atMost(nums, k) - atMost(nums, k - 1);
    }

private:
    int atMost(vector<int>& nums, int k) {
        int an = 0, sum = 0, l = 0;
        for (int i = 0; i < nums.size(); i++) {
            sum += nums[i] % 2 == 1;
            while (sum > k) sum -= nums[l++] % 2 == 1;
            an += i - l + 1;
        }
        return an;
    }
};
```
