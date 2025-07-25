# 421. Maximum XOR of Two Numbers in an Array

### Description

Given an integer array nums, return the maximum result of nums[i] XOR nums[j], where 0 <= i <= j < n.

### Example 

###### Example I

> Input: nums = [3,10,5,25,2,8]
> Output: 28
> Explanation: The maximum result is 5 XOR 25 = 28.

###### Example II

> Input: nums = [14,70,53,83,49,91,36,80,92,51,66,70]
> Output: 127

### Solution

先来一个暴力的：

```c++
class Solution {
public:
    int findMaximumXOR(vector<int>& nums) {
        int an = INT_MIN;
        int n = nums.size();
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                int current = nums[i] ^ nums[j];
                an = max(current, an);
            }
        }
        return an;
    }
};
```

接下来我们考虑时间复杂度优化一些的：

想要让异或操作之后的数字大，用二进制表示的数字串差异就要多，越多越好。

我们可以先假定某一个位可以为1，然后检查是否能为一，依据的逻辑是：x = a ^ b 等价于 a = x ^ b。

```c++
class Solution {
public:
    int findMaximumXOR(vector<int>& nums) {
        int an = 0;
        unordered_set<int> dict;
        for (int k = 30; k >= 0; k--) {
            for (int num : nums) dict.insert(num >> k);

            int an_next = 2 * an + 1;
            bool found = false;

            for (int num : nums) {
                if (dict.count(an_next ^ (num >> k)) != 0) {
                    found = true;
                }
            }

            an = found ? an_next : an_next - 1;
            dict.clear();
        }
        return an;
    }
};
```
 