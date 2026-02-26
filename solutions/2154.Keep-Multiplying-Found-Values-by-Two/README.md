# 2154. Keep Multiplying Found Values by Two

### Description

You are given an array of integers nums. You are also given an integer original which is the first number that needs to be searched for in nums.

You then do the following steps:

1. If original is found in nums, multiply it by two (i.e., set original = 2 * original).
2. Otherwise, stop the process.
3. Repeat this process with the new number as long as you keep finding the number.
Return the final value of original.

### Example

###### Example I

> Input: nums = [5,3,6,1,12], original = 3
> Output: 24
> Explanation: 
> - 3 is found in nums. 3 is multiplied by 2 to obtain 6.
> - 6 is found in nums. 6 is multiplied by 2 to obtain 12.
> - 12 is found in nums. 12 is multiplied by 2 to obtain 24.
> - 24 is not found in nums. Thus, 24 is returned.

###### Example II

> Input: nums = [2,7,9], original = 4
> Output: 4
> Explanation:
> - 4 is not found in nums. Thus, 4 is returned.

### Solution

遍历一遍数组，用哈希表记录所有出现过的数字。随后枚举可能的 Original。

```c++
class Solution {
public:
    int findFinalValue(vector<int>& nums, int original) {
        unordered_map<int, int> dict;
        for (int num : nums) dict[num]++;

        while (true) {
            if (dict.count(original) == 0) break;
            original *= 2;
        }
        return original;
    }
};
```

这里存在可以优化的地方：
- 我们可以只记录有可能的数字，他的形式应该是 original * 2^k。
- 我们甚至可以只把 k 记录下来，这时候只需要一个数字做位运算来存储就可以了。
