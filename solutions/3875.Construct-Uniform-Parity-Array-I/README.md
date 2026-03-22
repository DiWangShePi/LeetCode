# 3875. Construct Uniform Parity Array I

**Tags:** Math

### Description

You are given an array nums1 of n distinct integers.

You want to construct another array nums2 of length n such that the elements in nums2 are either all odd or all even.

For each index i, you must choose exactly one of the following (in any order):

- nums2[i] = nums1[i]
- nums2[i] = nums1[i] - nums1[j], for an index j != i
Return true if it is possible to construct such an array, otherwise, return false.

### Example

###### Example I

> Input: nums1 = [2,3]
> Output: true
> Explanation:
> Choose nums2[0] = nums1[0] - nums1[1] = 2 - 3 = -1.
> Choose nums2[1] = nums1[1] = 3.
> nums2 = [-1, 3], and both elements are odd. Thus, the answer is true​​​​​​​.

###### Example II

> Input: nums1 = [4,6]
> Output: true
> Explanation:​​​​​​​
> Choose nums2[0] = nums1[0] = 4.
> Choose nums2[1] = nums1[1] = 6.
> nums2 = [4, 6], and all elements are even. Thus, the answer is true.

### Solution

当全是奇数或者全是偶数时，条件自然能满足。当奇数偶数混杂时，任意奇数可以通过减去其他奇数来成为奇数，如果只存在一个奇数，则其余所有偶数可以减去这个奇数来成为奇数。所以所有情况下，条件都能满足。

```c++
class Solution {
public:
    bool uniformArray(vector<int>& nums1) {
        return true;
    }
};
```
