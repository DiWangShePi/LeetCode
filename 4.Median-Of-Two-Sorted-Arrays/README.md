# 4. Median Of Two Sorted Arrays

### 题目描述

> 给定两个大小为 m 和 n 的正序（从小到大）数组 nums1 和 nums2。
请你找出这两个正序数组的中位数，并且要求算法的时间复杂度为 O(log(m + n))。
你可以假设 nums1 和 nums2 不会同时为空。

**示例1:**

```
nums1 = [1, 3]
nums2 = [2]
    
则中位数是 2.0
```

**示例2:**

```
nums1 = [1, 2]
nums2 = [3, 4]
    
则中位数是 (2 + 3)/2 = 2.5
```

### 题目解析

假设m小于n，需要找到两个正序数组的中位数，假设指针A将数组m分为两份，指针B将数组n分为两份。
为获取中位数，令A+B = (m+n+1)/2，向下取整。
移动A和B，使得n[B] > m[A-1] 且 n[B-1] < m[A] (当A或者B触及边界时，上边界设为无穷大，下边界设为无穷小)
当m+n为奇数时，中位数为max(m[A-1], n[B-1])。当m+n为偶数时，中位数为(max(m[A-1], n[B-1]) + min(m[A] + n[B])) / 2。
初始化A=m/2，B = (m+n+1)/2 - A， 二分m即可找到临界位置。复杂度为log(m)。

### 代码实现

###### c++

```c++
#include <vector>
#include <iostream>
#include <climits>

using namespace std;

class Solution {
public:
    double findMedianSortedArrays(vector<int>& nums1, vector<int>& nums2) {
        if (nums1.size() > nums2.size()) {
            return findMedianSortedArrays(nums2, nums1);
        }

        int m = nums1.size(), n = nums2.size();
        int half = (m + n + 1) / 2;
        int left = 0, right = m;

        while (left <= right) {
            int pointer = (left + right) / 2;
            int j = half - pointer;

            int ALeft = (pointer == 0) ? INT_MIN : nums1[pointer - 1];
            int ARight = (pointer == m) ? INT_MAX : nums1[pointer];
            int BLeft = (j == 0) ? INT_MIN : nums2[j - 1];
            int BRight = (j == n) ? INT_MAX : nums2[j];

            if (ALeft <= BRight && BLeft <= ARight) {
                int maxLeft = max(ALeft, BLeft);
                int minRight = min(ARight, BRight);

                if ((m + n) % 2 == 0) {
                    return (maxLeft + minRight) / 2.0;
                } else {
                    return maxLeft;
                }
            } else if (ALeft > BRight) {
                right = pointer - 1;
            } else {
                left = pointer + 1;
            }
        }
    }
};
```