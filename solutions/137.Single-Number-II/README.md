# 137. Single Number II

### Description

Given an integer array nums where every element appears three times except for one, which appears exactly once. Find the single element and return it.

You must implement a solution with a linear runtime complexity and use only constant extra space.

### Solution

哈希表，作为第一种解法。

我们也可以考虑将每个数字以二进制的方式表示。
随后，我们定义xi代表最终答案在第i位上的二进制符号。
由于其他所有数字都出现3次，该位的元素之和要么为0要么为3，则该元素必定为所有位数之和模3的余数。
由此，我们可以逐步计算出最终答案在每一个位上的二进制表示，并获得最终数字。

> 至于门电路，赶紧爬

### Implementation

###### c++

```c++
class Solution {
public:
    int singleNumber(vector<int>& nums) {
        unordered_map<int, int> freq;
        for (int num: nums) {
            ++freq[num];
        }
        int ans = 0;
        for (auto [num, occ]: freq) {
            if (occ == 1) {
                ans = num;
                break;
            }
        }
        return ans;
    }
};
```