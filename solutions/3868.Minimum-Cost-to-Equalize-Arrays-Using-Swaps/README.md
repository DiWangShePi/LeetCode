# 3868. Minimum Cost to Equalize Arrays Using Swaps

**Tags:** 

### Description

You are given two integer arrays nums1 and nums2 of size n.

You can perform the following two operations any number of times on these two arrays:

- Swap within the same array: Choose two indices i and j. Then, choose either to swap nums1[i] and nums1[j], or nums2[i] and nums2[j]. This operation is free of charge.
- Swap between two arrays: Choose an index i. Then, swap nums1[i] and nums2[i]. This operation incurs a cost of 1.
Return an integer denoting the minimum cost to make nums1 and nums2 identical. If this is not possible, return -1.

### Example

###### Example I

> Input: nums1 = [10,20], nums2 = [20,10]
> Output: 0
> Explanation:
> Swap nums2[0] = 20 and nums2[1] = 10.
> nums2 becomes [10, 20].
> This operation is free of charge.
> nums1 and nums2 are now identical. The cost is 0.

###### Example II

> Input: nums1 = [10,10], nums2 = [20,20]
> Output: 1
> Explanation:
> Swap nums1[0] = 10 and nums2[0] = 20.
> nums1 becomes [20, 10].
> nums2 becomes [10, 20].
> This operation costs 1.
> Swap nums2[0] = 10 and nums2[1] = 20.
> nums2 becomes [20, 10].
> This operation is free of charge.
> nums1 and nums2 are now identical. The cost is 1.

###### Example III

> Input: nums1 = [10,20], nums2 = [30,40]
> Output: -1
> Explanation:
> It is impossible to make the two arrays identical. Therefore, the answer is -1.

### Solution

操作一可以直接无视掉，毕竟没有开销，只要包含的数字一样，排序一下就好了。

操作二的重点在于，两个数组中每个元素有多少个。如果是奇数个，那两个数组的不可能相同。如果是偶数个，则取两个的差值的一半作为相同的代价。

```c++
class Solution {
public:
    int minCost(vector<int>& nums1, vector<int>& nums2) {
        unordered_map<int, int> dict1, dict2;
        for (int& num : nums1) dict1[num]++;
        for (int& num : nums2) dict2[num]++;

        int cost = 0;
        for (const auto& [key, num] : dict1) {
            if (dict2.count(key) != 0) {
                if ((dict2[key] + num) % 2 != 0) return -1;
                cost += abs(num - dict2[key]) / 2;
            } else {
                if (num % 2 != 0) return -1;
                cost += num / 2;
            }
        }
        for (const auto& [key, num] : dict2) {
            if (dict1.count(key) != 0) continue;
            else {
                if (num % 2 != 0) return -1;
                cost += num / 2;
            }
        }
        return cost % 2 == 0 ? cost / 2 : -1;
    }
};
```
