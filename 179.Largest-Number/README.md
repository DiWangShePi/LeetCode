# 179. Largest Number

### Description

Given a list of non-negative integers nums, arrange them such that they form the largest number and return it.

Since the result may be very large, so you need to return a string instead of an integer.

### Solution

对数组进行排序，定义排序规则：若x+y>y+x，则x在y前面。

### Implmentation

###### c++

```c++
class Solution {
public:
    string largestNumber(vector<int>& nums) {
        sort(nums.begin(), nums.end(), [](const int &x, const int &y) {
            return to_string(x) + to_string(y) > to_string(y) + to_string(x);
        });

        if (nums[0] == 0) return "0";
        string an;
        for (int &n : nums) an += to_string(n);
        return an;
    }
};
```
