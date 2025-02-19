# 198. House Robber

### Description

You are a professional robber planning to rob houses along a street. Each house has a certain amount of money stashed, the only constraint stopping you from robbing each of them is that adjacent houses have security systems connected and it will automatically contact the police if two adjacent houses were broken into on the same night.

Given an integer array nums representing the amount of money of each house, return the maximum amount of money you can rob tonight without alerting the police.

### Solution

> 深度搜索嘛，每次不能只走一步，必须至少走两步。

上述的是典型的错误答案，很容易就超时了，也就数据量小的时候能用用。

我们需要的是动态规划，对于给定的第i间house，我们考虑抢不抢这间屋子的因素，应该是抢他的收益(抢第[i-2]间和这间的收益)，和不抢的收益(抢第[i-1]间的收益)谁更大。我们由此有了递推公式，就可以进行动态规划了。

### Implementation

###### c++

```c++
class Solution {
public:
    int rob(vector<int>& nums) {
        if (nums.size() == 1) return nums[0];
        if (nums.size() == 2) return max(nums[0], nums[1]);

        vector<int> rob;
        rob.push_back(nums[0]);
        rob.push_back(max(nums[0], nums[1]));
        for (int i = 2; i < nums.size(); i++) {
            rob.push_back(max(rob[i - 1], rob[i - 2] + nums[i]));
        }
        return rob.back();
    }
};
```
