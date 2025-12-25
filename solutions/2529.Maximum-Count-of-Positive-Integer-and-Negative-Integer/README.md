# 2529. Maximum Count of Positive Integer and Negative Integer

**Tags:** Binary Search

### Description

Given an array nums sorted in non-decreasing order, return the maximum between the number of positive integers and the number of negative integers.

- In other words, if the number of positive integers in nums is pos and the number of negative integers is neg, then return the maximum of pos and neg.
Note that 0 is neither positive nor negative.

### Example

###### Example I

> Input: nums = [-2,-1,-1,1,2,3]
> Output: 3
> Explanation: There are 3 positive integers and 3 negative integers. The maximum count among them is 3.

###### Example II

> Input: nums = [-3,-2,-1,0,0,1,2]
> Output: 3
> Explanation: There are 2 positive integers and 3 negative integers. The maximum count among them is 3.

###### Example III

> Input: nums = [5,20,66,1314]
> Output: 4
> Explanation: There are 4 positive integers and 0 negative integers. The maximum count among them is 4.

### Solution

二分查找找 0 的位置，没找到就代表就是分界点，找到了就往两边找小于 0 和大于 0 的第一个数字。

```c++
class Solution {
public:
    int maximumCount(vector<int>& nums) {
        int index = find(nums);
        int l = index - 1, r = index;
        if (index > -1 && index < nums.size() && nums[index] == 0) {
            while (l > -1 && nums[l] == 0) l--;
            while (r < nums.size() && nums[r] == 0) r++;
        } 
        // cout << l << " " << r << endl;
        int n = nums.size() - r;
        return max(l + 1, n);
    }

private:
    int find(vector<int>& nums) {
        int l = 0, r = nums.size() - 1;
        while (l <= r) {
            int mid = l + (r - l) / 2;
            if (nums[mid] == 0) return mid;
            else if (nums[mid] > 0) r = mid - 1;
            else l = mid + 1;
        }
        return l;
    }
};
```
