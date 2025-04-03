# 260. Single Number III

### Description

Given an integer array nums, in which exactly two elements appear only once and all the other elements appear exactly twice. Find the two elements that appear only once. You can return the answer in any order.

You must write an algorithm that runs in linear runtime complexity and uses only constant extra space.

### Solution

如果允许使用线性的额外时间，这一题就十分简单了：遍历，将出现次数存储在一个字典中，再遍历字典找到仅出现一次的数字。

```c++
class Solution {
public:
    vector<int> singleNumber(vector<int>& nums) {
        unordered_map<int, int> dict;
        for (int i = 0; i < nums.size(); i++) dict[nums[i]]++;

        vector<int> result;
        for (auto x : dict) if (x.second == 1) result.push_back(x.first);
        return result;
    }
};
```

不过题目希望我们使用常数的额外时间，因此我们考虑使用xor这一操作符：相同的数字会通过xor变为0，最后剩下的数字是两个仅出现一次的数字的xor结果。

$$
1 ^ 2 ^ 1 ^ 3 ^ 2 ^ 5 = (1 ^ 1) ^ (2 ^ 2) ^ (3 ^ 5) = 0 ^ 0 ^ 6 = 6
$$

6的二进制表达是0110，第二位就是第一个代表3和5二进制表达不同的数位，我们通过将6与-6进行一次与操作将他提取出来。
随后，我们用这个数字将原有的nums数组分为两部分，再在每一部分中进行一次遍历xor，即可分别获得两个数字。

```c++
class Solution {
public:
    vector<int> singleNumber(vector<int>& nums) {
        unsigned int xor_all = 0;
        for (int num : nums) xor_all ^= num;
        int bit_diff = xor_all & -xor_all;

        int x = 0, y = 0;
        for (int num : nums) {
            if (num & bit_diff) x ^= num;
            else y ^= num;
        }
        return {x, y};
    }
};
```
