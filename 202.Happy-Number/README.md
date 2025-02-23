# 202. Happy Number

### Description

Write an algorithm to determine if a number n is happy.

A happy number is a number defined by the following process:

- Starting with any positive integer, replace the number by the sum of the squares of its digits.
- Repeat the process until the number equals 1 (where it will stay), or it loops endlessly in a cycle which does not include 1.
- Those numbers for which this process ends in 1 are happy.
Return true if n is a happy number, and false if not.

### Solution

按照要求重复计算，检查新的值是否为1。一个额外的字典检查当前值此前是否出现过，出现则陷入循环，返回false。遇到1即返回True。

### Implementation

###### c++

```c++
class Solution {
public:
    bool isHappy(int n) {
        unordered_map<int, bool> dict;
        vector<int> nums;
        while(true) {
            spilt(nums, n);
            int current = 0;
            for (int i = 0; i < nums.size(); i++) {
                current += nums[i] * nums[i];
            }
            if (current == 1) return true;
            if (dict.count(current) != 0) return false;
            dict[current] = true;
            n = current;
        }
        return false;
    }

private:
    void spilt(vector<int>& nums, int num) {
        nums.clear();
        while (num != 0) {
            int mid = num % 10;
            nums.push_back(mid);
            num = num / 10;
        }
    }
};
```
