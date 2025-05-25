# 327. Count of Range Sum

### Description

Given an integer array nums and two integers lower and upper, return the number of range sums that lie in [lower, upper] inclusive.

Range sum S(i, j) is defined as the sum of the elements in nums between indices i and j inclusive, where i <= j.

### Example 

###### Example I

```
Input: nums = [-2,5,-1], lower = -2, upper = 2
Output: 3
Explanation: The three ranges are: [0,0], [2,2], and [0,2] and their respective sums are: -2, -1, 2.
```

###### Example II

```
Input: nums = [0], lower = 0, upper = 0
Output: 1
```

### Solution

先考虑暴力的做法：计算给定数组`nums`的前缀和，给定下标`i`和`j`，计算这个范围内的数字之和，检查是否处于`lower`和`upper`之间。

```c++
class Solution {
public:
    int countRangeSum(vector<int>& nums, int lower, int upper) {
        int n = nums.size();
        if (n == 0) return 0;

        vector<long long> sums{nums[0]};
        for (int i = 1; i < n; i++) sums.push_back(nums[i] + sums[i - 1]);

        int an = 0;
        for (int i = 0; i < n; i++) {
            for (int j = i; j < n; j++) {
                long long result = calculate(sums, i, j);

                if (result >= lower && result <= upper) an++;
            }
        }
        return an;
    }

private:
    long long calculate(vector<long long>& sums, int i, int j) {
        if (i == 0) return sums[j];
        return sums[j] - sums[i - 1];
    }
};
```

> 当然，这个解法被ban了

我们还可以用归并排序的方式来解决

```c++
class Solution {
public:
    int countRangeSum(vector<int>& nums, int lower, int upper) {
        int n = nums.size();
        vector<long long> prefixSums(n + 1, 0);
        
        for (int i = 0; i < n; ++i) {
            prefixSums[i + 1] = prefixSums[i] + nums[i];
        }

        return mergeSort(prefixSums, 0, n + 1, lower, upper);
    }

private:
    int mergeSort(vector<long long>& sums, int left, int right, int lower, int upper) {
        if (right - left <= 1) return 0;

        int mid = (left + right) / 2;
        int count = mergeSort(sums, left, mid, lower, upper) +
                    mergeSort(sums, mid, right, lower, upper);

        int j = mid, k = mid, t = mid;
        vector<long long> temp;
        
        for (int i = left; i < mid; ++i) {
            // k: first index >= sums[i] + lower
            while (k < right && sums[k] - sums[i] < lower) k++;
            // j: first index > sums[i] + upper
            while (j < right && sums[j] - sums[i] <= upper) j++;
            count += j - k;

            // Merge step: keep sorted
            while (t < right && sums[t] < sums[i]) temp.push_back(sums[t++]);
            temp.push_back(sums[i]);
        }

        // Append remaining elements
        while (t < right) temp.push_back(sums[t++]);

        // Copy back
        for (int i = 0; i < temp.size(); ++i) {
            sums[left + i] = temp[i];
        }

        return count;
    }
};
```
