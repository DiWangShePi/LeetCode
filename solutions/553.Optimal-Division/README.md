# 553. Optimal Division

**Tags:** String, Math

### Description

You are given an integer array nums. The adjacent integers in nums will perform the float division.

- For example, for nums = [2,3,4], we will evaluate the expression "2/3/4".
However, you can add any number of parenthesis at any position to change the priority of operations. You want to add these parentheses such the value of the expression after the evaluation is maximum.

Return the corresponding expression that has the maximum value in string format.

Note: your expression should not contain redundant parenthesis.

### Example

###### Example I

> Input: nums = [1000,100,10,2]
> Output: "1000/(100/10/2)"
> Explanation: 1000/(100/10/2) = 1000/((100/10)/2) = 200
> However, the bold parenthesis in "1000/((100/10)/2)" are redundant since they do not influence the operation priority.
> So you should return "1000/(100/10/2)".
> Other cases:
> 1000/(100/10)/2 = 50
> 1000/(100/(10/2)) = 50
> 1000/100/10/2 = 0.5
> 1000/100/(10/2) = 2

###### Example II

> Input: nums = [2,3,4]
> Output: "2/(3/4)"
> Explanation: (2/(3/4)) = 8/3 = 2.667
> It can be shown that after trying all possibilities, we cannot get an expression with evaluation greater than 2.667

### Solution

为了使最终的结果最大，我们希望分子尽可能大，分母尽可能小。

对于给定的序列，由于只能用除法，nums[0]就是可能的最大值了。为了让分子最小，我们要让后面的一路除下去。

```c++
class Solution {
public:
    string optimalDivision(vector<int>& nums) {
        string an = to_string(nums[0]);
        if (nums.size() == 1) return an;

        an += "/";
        if (nums.size() == 2) return an + to_string(nums[1]);

        an += "(";
        for (int i = 1; i < nums.size(); i++) {
            an += to_string(nums[i]);
            an += "/";
        }
        an.pop_back();
        return an + ")";
    }
};
```
