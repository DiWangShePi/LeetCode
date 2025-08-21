# 453. Minimum Moves to Equal Array Elements

### Description

Given an integer array nums of size n, return the minimum number of moves required to make all array elements equal.

In one move, you can increment n - 1 elements of the array by 1.

### Example

###### Example I

> Input: nums = [1,2,3]
> Output: 3
> Explanation: Only three moves are needed (remember each move increments two elements):
> [1,2,3]  =>  [2,3,3]  =>  [3,4,3]  =>  [4,4,4]

###### Example II

> Input: nums = [1,1,1]
> Output: 0

### Solution

最终的目标是让所有的元素一致，因此我们不需要关心最终的值是多少，只需要关心相对值是多少。

此外，每一次将n-1的数字加一，也可以理解为将一个数字减一。由此，我们将这个问题转化为：需要多少次操作，将每个数字变得等同于最小值。

```c++
class Solution {
public:
    int minMoves(vector<int>& nums) {
        int minNum = *min_element(nums.begin(), nums.end());
        long long numSum = accumulate(nums.begin(), nums.end(), 0LL);
        return numSum - minNum * nums.size();
    }
};
```
