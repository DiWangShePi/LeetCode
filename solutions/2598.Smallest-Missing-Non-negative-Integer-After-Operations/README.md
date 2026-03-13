# 2598. Smallest Missing Non-negative Integer After Operations

**Tags:** Math

### Description

You are given a 0-indexed integer array nums and an integer value.

In one operation, you can add or subtract value from any element of nums.

- For example, if nums = [1,2,3] and value = 2, you can choose to subtract value from nums[0] to make nums = [-1,2,3].
The MEX (minimum excluded) of an array is the smallest missing non-negative integer in it.

- For example, the MEX of [-1,2,3] is 0 while the MEX of [1,0,3] is 2.
Return the maximum MEX of nums after applying the mentioned operation any number of times.

### Example

###### Example I

> Input: nums = [1,-10,7,13,6,8], value = 5
> Output: 4
> Explanation: One can achieve this result by applying the following operations:
> - Add value to nums[1] twice to make nums = [1,0,7,13,6,8]
> - Subtract value from nums[2] once to make nums = [1,0,2,13,6,8]
> - Subtract value from nums[3] twice to make nums = [1,0,2,3,6,8]
> The MEX of nums is 4. It can be shown that 4 is the maximum MEX we can achieve.

###### Example II

> Input: nums = [1,-10,7,13,6,8], value = 7
> Output: 2
> Explanation: One can achieve this result by applying the following operation:
> - subtract value from nums[2] once to make nums = [1,-10,0,13,6,8]
> The MEX of nums is 2. It can be shown that 2 is the maximum MEX we can achieve.

### Solution

由于可以执行任意次数的操作，而且可加可减，因此我们只需要知道每个给定的 num 值能因 value 对应到哪些数字上（取模）即可。

```c++
class Solution {
public:
    int findSmallestInteger(vector<int>& nums, int value) {
        unordered_map<int, int> dict;
        for (int& num : nums) {
            int t = num % value;
            if (t < 0) t += value;
            dict[t]++;
        }

        int an = 0;
        while (dict.size() != 0) {
            int t = an % value;
            if (t < 0) t += value;
            if (dict.count(t) != 0) {
                dict[t]--;
                if (dict[t] == 0) dict.erase(t);
                an++;
            } else break;
        }
        return an;
    }
};
```
