# 493. Reverse Pairs

**Tags:** Merge Sort

### Description

Given an integer array nums, return the number of reverse pairs in the array.

A reverse pair is a pair (i, j) where:

- 0 <= i < j < nums.length and
- nums[i] > 2 * nums[j].

### Example

###### Example I

> Input: nums = [1,3,2,3,1]
> Output: 2
> Explanation: The reverse pairs are:
> (1, 4) --> nums[1] = 3, nums[4] = 1, 3 > 2 * 1
> (3, 4) --> nums[3] = 3, nums[4] = 1, 3 > 2 * 1

###### Example II

> Input: nums = [2,4,3,5,1]
> Output: 3
> Explanation: The reverse pairs are:
> (1, 4) --> nums[1] = 4, nums[4] = 1, 4 > 2 * 1
> (2, 4) --> nums[2] = 3, nums[4] = 1, 3 > 2 * 1
> (3, 4) --> nums[3] = 5, nums[4] = 1, 5 > 2 * 1

### Solution

先来暴力的枚举检验

```c++
class Solution {
public:
    int reversePairs(vector<int>& nums) {
        int an = 0;
        for (int i = 0; i < nums.size(); i++) {
            for (int j = i + 1; j < nums.size(); j++) {
                if (nums[i] > (long long) 2 * nums[j]) an++;
            }
        }
        return an;
    }
};
```

更优的解法是归并排序，每次对子数组的元素计算符号要求的值

```c++
class Solution {
public:
    int reversePairs(vector<int>& nums) {
        return mergeSortAndCount(nums, 0, nums.size() - 1);
    }

private:

    int mergeSortAndCount(vector<int>& nums, int left, int right) {
        if (left >= right) return 0;
        
        int mid = left + (right - left) / 2;
        int count = mergeSortAndCount(nums, left, mid) + mergeSortAndCount(nums, mid + 1, right);
        
        count += countPairs(nums, left, mid, right);
        merge(nums, left, mid, right);
        return count;
    }

    int countPairs(vector<int>& nums, int left, int mid, int right) {
        int count = 0, j = mid + 1;
        
        for (int i = left; i <= mid; i++) {
            while (j <= right && (long) nums[i] > 2L * nums[j]) j++;
            count += (j - (mid + 1));
        }
        return count;
    }

    void merge(vector<int>& nums, int left, int mid, int right) {
        vector<int> temp(right - left + 1);
        int i = left, j = mid + 1, k = 0;

        while (i <= mid && j <= right) {
            if (nums[i] <= nums[j]) temp[k++] = nums[i++];
            else temp[k++] = nums[j++];
        }
        while (i <= mid) temp[k++] = nums[i++];
        while (j <= right) temp[k++] = nums[j++];

        for (int p = 0; p < temp.size(); p++)
            nums[left + p] = temp[p];
    }
};
```
