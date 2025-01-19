# 169. Majority Element

### Description

Given an array nums of size n, return the majority element.

The majority element is the element that appears more than ⌊n / 2⌋ times. You may assume that the majority element always exists in the array.

### Solution

等了很久的题，居然只是一道简单题，当年觉得解法挺有意思的。

一个变量保存当前多数数字，另一个变量保存遇见该数字的次数。

若新数字与多数数字相同，则遇见次数加一。不相同则减一。为零时变为当前的多数数字。

### Implementation

###### c++

```c++
class Solution {
public:
    int majorityElement(vector<int>& nums) {
        int an = nums[0];
        int num = 0;
        for (int i = 1; i < nums.size(); i++) {
            if (nums[i] == an) num++;
            else {
                if (num == 0) an = nums[i];
                else num--;
            }
        }
        return an;
    }
};
```
