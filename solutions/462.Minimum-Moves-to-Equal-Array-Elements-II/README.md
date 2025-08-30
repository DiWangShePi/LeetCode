# 462. Minimum Moves to Equal Array Elements II

### Description

Given an integer array nums of size n, return the minimum number of moves required to make all array elements equal.

In one move, you can increment or decrement an element of the array by 1.

Test cases are designed so that the answer will fit in a 32-bit integer.

### Example

###### Example I

> Input: nums = [1,2,3]
> Output: 2
> Explanation:
> Only two moves are needed (remember each move increments or decrements one element):
> [1,2,3]  =>  [2,2,3]  =>  [2,2,2]

###### Example II

> Input: nums = [1,10,2,9]
> Output: 16

### Solution

排序后找中间值，往这个值走的开销最小

```c++
class Solution {
public:
    int minMoves2(vector<int>& nums) {
        sort(nums.begin(), nums.end());

        int an = 0;
        for (int val : nums) an += abs(val - nums[nums.size() / 2]);
        return an;
    }
};
```
