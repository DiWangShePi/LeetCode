# 485. Max Consecutive Ones

### Description

Given a binary array nums, return the maximum number of consecutive 1's in the array.

### Example 

###### Example I

> Input: nums = [1,1,0,1,1,1]
> Output: 3
> Explanation: The first two digits or the last three digits are consecutive 1s. The maximum number of consecutive 1s is 3.

###### Example II

> Input: nums = [1,0,1,1,0,1]
> Output: 2

### Solution

按要求实现即可

```c++
class Solution {
public:
    int findMaxConsecutiveOnes(vector<int>& nums) {
        int an = 0, temp = 0;
        for (int num : nums) {
            if (num == 1) temp++;
            else {
                an = max(an, temp);
                temp = 0;
            }
        }
        return max(an, temp);
    }
};
```
