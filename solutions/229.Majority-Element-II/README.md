# 229. Majority Element II

### Description

Given an integer array of size n, find all elements that appear more than ⌊ n/3 ⌋ times.

### Solution

第一种方法是遍历，数一遍每种元素出现了多少次，然后检查并将满足条件的数字纳入答案。

第二种方法是Boyer-Moore Voting Algorithm。

### Implementation

###### c++

> 我不理解的是这里遍历两遍为什么会比用字典遍历两遍要快

```c++
class Solution {
public:
    vector<int> majorityElement(vector<int>& nums) {
        int c1 = INT_MAX, c2 = INT_MAX, count1 = 0, count2 = 0;
        for (int val : nums) {
            if (val == c1) count1++;
            else if (val == c2) count2++;
            else if (count1 == 0) {
                c1 = val;
                count1 = 1;
            } else if (count2 == 0) {
                c2 = val;
                count2 = 1;
            } else {
                count1--;
                count2--;
            }
        }

        count1 = count2 = 0;
        for (int val : nums) {
            if (val == c1) count1++;
            else if (val == c2) count2++;
        }

        vector<int> result;
        int threshold = nums.size() / 3;
        if (count1 > threshold) result.push_back(c1);
        if (count2 > threshold) result.push_back(c2);

        return result;
    }
};
```
