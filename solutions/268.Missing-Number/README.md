# 268. Missing Number

### Description

Given an array nums containing n distinct numbers in the range [0, n], return the only number in the range that is missing from the array.

### Solution

第一个想法是遍历两遍：

```c++
class Solution {
public:
    int missingNumber(vector<int>& nums) {
        unordered_map<int, int> dict;
        for (int i = 0; i < nums.size(); i++) dict[nums[i]] = 1;
        for (int i = 0; i <= nums.size(); i++) {
            if (dict.count(i) == 0) return i;
        }
        return 0;
    }
};
```

随后发现，既然只会出现这个范围的数字且只出现一次，那么就可以先计算出这个范围所有数字之和，与给定数组的实际和比对，即可知晓谁不见了。

```c++
class Solution {
public:
    int missingNumber(vector<int>& nums) {
        int sum = nums.size() * (nums.size() + 1) / 2;
        int actual_sum = 0;
        for (int i = 0; i < nums.size(); i++) actual_sum += nums[i];
        return sum - actual_sum;
    }
};
```
