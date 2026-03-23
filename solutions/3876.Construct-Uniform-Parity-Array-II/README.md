# 3876. Construct Uniform Parity Array II

**Tags:** Math

### Description

You are given an array nums1 of n distinct integers.

You want to construct another array nums2 of length n such that the elements in nums2 are either all odd or all even.

For each index i, you must choose exactly one of the following (in any order):

- nums2[i] = nums1[i]​​​​​​​
- nums2[i] = nums1[i] - nums1[j], for an index j != i, such that nums1[i] - nums1[j] >= 1
Return true if it is possible to construct such an array, otherwise return false.

### Example

###### Example I

> Input: nums1 = [1,4,7]
> Output: true
> Explanation:​​​​​​​​​​​​​​
> Set nums2[0] = nums1[0] = 1.
> Set nums2[1] = nums1[1] - nums1[0] = 4 - 1 = 3.
> Set nums2[2] = nums1[2] = 7.
> nums2 = [1, 3, 7], and all elements are odd. Thus, the answer is true.

###### Example II

> Input: nums1 = [2,3]
> Output: false
> Explanation:
> It is not possible to construct nums2 such that all elements have the same parity. Thus, the answer is false.

###### Example III

> Input: nums1 = [4,6]
> Output: true
> Explanation:
> Set nums2[0] = nums1[0] = 4.
> Set nums2[1] = nums1[1] = 6.
> nums2 = [4, 6], and all elements are even. Thus, the answer is true.

### Solution

奇数遇到奇数可以变成偶数，偶数遇到奇数也可以变成奇数。按照题目要求，每个数字可以保持自身不变，也可以通过减去一个比自己小的数字来变成另一个数字。

因此，这个数组最终能变成全是偶数还是全是奇数，取决于数组中最小的数字。如果这个数字是奇数，整个数组只能都变成奇数。如果是偶数，那就都是偶数。

如果最小的值是奇数，后面的任意奇数可以保持不变，后面的任意偶数可以减去这个奇数来变成奇数。

如果最小的值是偶数，那后面的奇数总会有最小哪一个无法变成偶数，因此需要所有的数字都是奇数。

```c++
class Solution {
public:
    bool uniformArray(vector<int>& nums1) {
        sort(nums1.begin(), nums1.end());
        int mn = nums1[0];

        bool hasOdd = false;
        for (int& num : nums1) {
            mn = min(mn, num);
            if(num & 1) hasOdd = true;
        }
        return (mn & 1) || !hasOdd;
    }
};
```
