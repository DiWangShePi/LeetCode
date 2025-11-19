# 2090. K Radius Subarray Averages

**Tags:** Sliding Window

### Description

You are given a 0-indexed array nums of n integers, and an integer k.

The k-radius average for a subarray of nums centered at some index i with the radius k is the average of all elements in nums between the indices i - k and i + k (inclusive). If there are less than k elements before or after the index i, then the k-radius average is -1.

Build and return an array avgs of length n where avgs[i] is the k-radius average for the subarray centered at index i.

The average of x elements is the sum of the x elements divided by x, using integer division. The integer division truncates toward zero, which means losing its fractional part.

- For example, the average of four elements 2, 3, 1, and 5 is (2 + 3 + 1 + 5) / 4 = 11 / 4 = 2.75, which truncates to 2.

### Example

###### Example I

![](./eg1.png)

> Input: nums = [7,4,3,9,1,8,5,2,6], k = 3
> Output: [-1,-1,-1,5,4,4,-1,-1,-1]
> Explanation:
> - avg[0], avg[1], and avg[2] are -1 because there are less than k elements before each index.
> - The sum of the subarray centered at index 3 with radius 3 is: 7 + 4 + 3 + 9 + 1 + 8 + 5 = 37.
>   Using integer division, avg[3] = 37 / 7 = 5.
> - For the subarray centered at index 4, avg[4] = (4 + 3 + 9 + 1 + 8 + 5 + 2) / 7 = 4.
> - For the subarray centered at index 5, avg[5] = (3 + 9 + 1 + 8 + 5 + 2 + 6) / 7 = 4.
> - avg[6], avg[7], and avg[8] are -1 because there are less than k elements after each index.

###### Example II

> Input: nums = [100000], k = 0
> Output: [100000]
> Explanation:
> - The sum of the subarray centered at index 0 with radius 0 is: 100000.
>   avg[0] = 100000 / 1 = 100000.

###### Example III

> Input: nums = [8], k = 100000
> Output: [-1]
> Explanation: 
> - avg[0] is -1 because there are less than k elements before and after index 0.

### Solution

稍微复杂一点的滑动窗口，参见：https://leetcode.cn/discuss/post/3578981/ti-dan-hua-dong-chuang-kou-ding-chang-bu-rzz7/

```c++
class Solution {
public:
    vector<int> getAverages(vector<int>& nums, int k) {
        int n = nums.size();
        if (k > n / 2) return vector<int>(n, -1);

        vector<int> an;
        long long index = 0, current = 0;
        while (index < 2 * k) {
            current += nums[index];
            if (index >= k) an.push_back(-1);

            index++;
        }
        while (index < n) {
            current += nums[index];
            an.push_back(current / (2 * k + 1));
            current -= nums[index - 2 * k];

            index++;
        }
        while (index - k < n) {
            an.push_back(-1);

            index++;
        }
        return an;
    }
};
```
