# 3886. Sum of Sortable Integers

**Tags:**

### Description

You are given an integer array nums of length n.

An integer k is called sortable if k divides n and you can sort nums in non-decreasing order by sequentially performing the following operations:

- Partition nums into consecutive subarrays of length k.
- Cyclically rotate each subarray independently any number of times to the left or to the right.
Return an integer denoting the sum of all possible sortable integers k.

### Example

###### Example I

> Input: nums = [3,1,2]
> Output: 3
> Explanation:​​​​​​​
> For n = 3, possible divisors are 1 and 3.
> For k = 1: each subarray has one element. No rotation can sort the array.
> For k = 3: the single subarray [3, 1, 2] can be rotated once to produce [1, 2, 3], which is sorted.
> Only k = 3 is sortable. Hence, the answer is 3.

###### Example II

> Input: nums = [7,6,5]
> Output: 0
> Explanation:
> For n = 3, possible divisors are 1 and 3.
> For k = 1: each subarray has one element. No rotation can sort the array.
> For k = 3: the single subarray [7, 6, 5] cannot be rotated into non-decreasing order.
> No k is sortable. Hence, the answer is 0.

###### Example III

> Input: nums = [5,8]
> Output: 3
> Explanation:​​​​​​​
> For n = 2, possible divisors are 1 and 2.
> Since [5, 8] is already sorted, every divisor is sortable. Hence, the answer is 1 + 2 = 3.

### Solution

对于每一个数组，如果它可以通过旋转来变为有序的，那么这个数组中一定只存在不多于一个点位，使得 A[i] > A[i + 1]

```c++
class Solution {
public:
    int sortableIntegers(vector<int>& nums) {
        int n = nums.size(), an = 0;
        for (int i = 1; i <= n; i++) {
            if (n % i != 0) continue;

            int k = n / i;
            bool could = true;
            vector<vector<int>> checkList;
            for (int j = 0; j < n; j += k) {
                vector<int> c = check(nums, j, j + k);
                if (c[0] > 1) {
                    could = false;
                    break;
                }
                checkList.push_back(c);
            }
            if (could) {
                for (int j = 0; j < checkList.size() - 1; j++) {
                    if (checkList[j][1] > checkList[j + 1][2]) {
                        could = false;
                        break;
                    }
                }
            }
            cout << i << " " << could << endl;
            if (could) an += k;
        }
        return an;
    }

private:
    vector<int> check(vector<int>& nums, int l, int r) {
        int count = 0, high = nums[r - 1], low = nums[r - 1];
        for (int i = l; i < r - 1; i++) {
            if (nums[i] > nums[i + 1]) count++;
            high = max(high, nums[i]);
            low = min(low, nums[i]);
        }
        if (count == 1 && nums[r - 1] > nums[l]) count++;
        return {count, high, low};
    }
};
```
