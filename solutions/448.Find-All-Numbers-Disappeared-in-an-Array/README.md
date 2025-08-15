# 448. Find All Numbers Disappeared in an Array

**Tags:** Hash Table,  

### Description

Given an array nums of n integers where nums[i] is in the range [1, n], return an array of all the integers in the range [1, n] that do not appear in nums.

### Example

###### Example I

> Input: nums = [4,3,2,7,8,2,3,1]
> Output: [5,6]

###### Example II

> Input: nums = [1,1]
> Output: [2]

### Solution

先给一个暴力的解法，用哈希表。遍历数组并记录所有出现过的元素，随后遍历1到n的所有数字，检查是否出现过。

```c++
class Solution {
public:
    vector<int> findDisappearedNumbers(vector<int>& nums) {
        unordered_map<int, int> dict;
        for (int num : nums) dict[num]++;

        vector<int> an;
        for (int i = 1; i <= nums.size(); i++) {
            if (dict.count(i) == 0) 
                an.push_back(i);
        }
        return an;
    }
};
```

但这需要一个额外的哈希表，在最劣的情况下额外空间复杂度是O(n)的。

由于给定的数字都是正数，我们也可以将数字的正或负表示下标数字是否出现。

```c++
class Solution {
public:
    vector<int> findDisappearedNumbers(vector<int>& nums) {
        for (int i = 0; i < nums.size(); i++) {
            int c = nums[i] > 0 ? nums[i] : -nums[i];
            nums[c - 1] = nums[c - 1] > 0 ? -nums[c - 1] : nums[c - 1];
        }

        vector<int> an;
        for (int i = 0; i < nums.size(); i++) {
            if (nums[i] > 0) an.push_back(i + 1);
        }
        return an;
    }
};
```

