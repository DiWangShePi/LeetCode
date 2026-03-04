# 565. Array Nesting

### Description

You are given an integer array nums of length n where nums is a permutation of the numbers in the range [0, n - 1].

You should build a set s[k] = {nums[k], nums[nums[k]], nums[nums[nums[k]]], ... } subjected to the following rule:

- The first element in s[k] starts with the selection of the element nums[k] of index = k.
- The next element in s[k] should be nums[nums[k]], and then nums[nums[nums[k]]], and so on.
- We stop adding right before a duplicate element occurs in s[k].
Return the longest length of a set s[k].

### Example

###### Example I

> Input: nums = [5,4,0,3,1,6,2]
> Output: 4
> Explanation: 
> nums[0] = 5, nums[1] = 4, nums[2] = 0, nums[3] = 3, nums[4] = 1, nums[5] = 6, nums[6] = 2.
> One of the longest sets s[k]:
> s[0] = {nums[0], nums[5], nums[6], nums[2]} = {5, 6, 2, 0}

###### Example II

> Input: nums = [0,1,2]
> Output: 1

### Solution

记录一下每个元素是否在之前的尝试中被访问过了。如果已经被访问过了，那么它就可以被用于构造一个更长的 path，不用在此时重新遍历了。

```c++
class Solution {
public:
    int arrayNesting(vector<int>& nums) {
        int n = nums.size();
        vector<bool> visited(n, false);
        int an = 0;

        for (int i = 0; i < n; i++) {
            if (!visited[i]) {
                int index = i, c = 0;

                while (!visited[index]) {
                    visited[index] = true;
                    index = nums[index];
                    c++;
                }
                an = max(an, c);
            }
        }
        return an;
    }
};
```
