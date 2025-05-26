# 330. Patching Array

### Description

Given a sorted integer array nums and an integer n, add/patch elements to the array such that any number in the range [1, n] inclusive can be formed by the sum of some elements in the array.

Return the minimum number of patches required.

### Example 

###### Example I

```
Input: nums = [1,3], n = 6
Output: 1
Explanation:
Combinations of nums are [1], [3], [1,3], which form possible sums of: 1, 3, 4.
Now if we add/patch 2 to nums, the combinations are: [1], [2], [3], [1,3], [2,3], [1,2,3].
Possible sums are 1, 2, 3, 4, 5, 6, which now covers the range [1, 6].
So we only need 1 patch.
```

###### Example II

```
Input: nums = [1,5,10], n = 20
Output: 2
Explanation: The two patches can be [2, 4].
```

###### Example III

```
Input: nums = [1,2,2], n = 5
Output: 0
```

### Solution

对于正整数x，如果区间[1,x-1]内的所有数字都已经被覆盖，且x在数组中，那么区间[1,2x-1]内的所有数字也都被覆盖。如果不存在x，那么至少需要在数组中补充一个小于或等于x的数字，才能覆盖到x。

如果区间[1,x-1]内的所有数字都已经被覆盖，则从贪心的角度考虑，补充x后即可覆盖x，且满足补充的数字个数最少。在补充x之后，区间[1,2x-1]内的数字同样都被覆盖，下一个确实的数字一定不会小于2x。

由此可以采用贪心的方式：每次找到未被数组nums覆盖的最小整数x，在数组中补充x，然后寻找下一个未被覆盖的最小整数，重复直到[1,n]中的所有数组都被覆盖。

```c++
class Solution {
public:
    int minPatches(vector<int>& nums, int n) {
        int patches = 0;
        long long x = 1;
        int length = nums.size(), index = 0;
        while (x <= n) {
            if (index < length && nums[index] <= x) {
                x += nums[index];
                index++;
            } else {
                x <<= 1;
                patches++;
            }
        }
        return patches;
    }
};
```
