# 31. Next Permutation

### 题目描述

整数数组的排列是将其成员排列成序列或线性顺序。

- 例如，对于arr =[1,2,3]，以下是arr的所有排列:[1,2,3]，[1,3,2]，[2,1,3]，[2,3,1]，[3,1,2]，[3,2,1]。

整数数组的下一个排列是该整数在字典顺序上的下一个更大的排列。更正式地说，如果数组的所有排列都按照字典顺序在一个容器中排序，那么该数组的下一个排列就是在排序后的容器中紧随其后的排列。如果这样的排列是不可能的，数组必须重新排列为最低可能的顺序(即，按升序排序)。

- 例如，arr =[1,2,3]的下一个排列是[1,3,2]。

- 类似地，arr =[2,3,1]的下一个排列是[3,1,2]。

- 而arr =[3,2,1]的下一个排列是[1,2,3]，因为[3,2,1]在字典顺序上没有更大的重排。

给定一个整数数组nums，找出nums的下一个排列。替换必须到位，并且只使用常数级的额外内存。

### 题目解析

从后向前遍历字符串，找到第一个位置i，使得array[i] < array[i+1]。
如果找不到符合的条件，这代表整个数组是降序排列的，将数组按升序排序后返回。
如果找到了指定的i，则在(i+1, array.length-1)中找到大于array[i]的最小值，将两者交换。
随后，将(i+1, array.length-1)部分的数字按升序排序。

### 代码实现

###### c++

```c++
class Solution {
public:
    void nextPermutation(vector<int>& nums) {
        int n = nums.size();
        if (n < 2) return;

        int point = n - 2;
        while (point >= 0 && nums[point] >= nums[point + 1]) {
            point--;
        }

        if (point >= 0) {
            int larger = n - 1;
            while (nums[larger] <= nums[point]) {
                larger--;
            }
            std::swap(nums[point], nums[larger]);
        }

        std::reverse(nums.begin() + point + 1, nums.end());
    }
};
```