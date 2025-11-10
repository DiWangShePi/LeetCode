# 1343. Number of Sub-arrays of Size K and Average Greater than or Equal to Threshold

**Tags:** Sliding Window

### Description

Given an array of integers arr and two integers k and threshold, return the number of sub-arrays of size k and average greater than or equal to threshold.

### Example

###### Example I

> Input: arr = [2,2,2,2,5,5,5,8], k = 3, threshold = 4
> Output: 3
> Explanation: Sub-arrays [2,5,5],[5,5,5] and [5,5,8] have averages 4, 5 and 6 respectively. All other sub-arrays of size 3 have averages less than 4 (the threshold).

###### Example II

> Input: arr = [11,13,17,23,29,31,7,5,2,3], k = 3, threshold = 5
> Output: 6
> Explanation: The first 6 sub-arrays of size 3 have averages greater than 5. Note that averages are not integers.

### Solution

滑动窗口，参见：https://leetcode.cn/discuss/post/3578981/ti-dan-hua-dong-chuang-kou-ding-chang-bu-rzz7/

```c++
class Solution {
public:
    int numOfSubarrays(vector<int>& arr, int k, int threshold) {
        int current = 0, an = 0, index = 0;
        while (index < arr.size()) {
            current += arr[index];
            index++;
            if (index >= k) {
                if (current / k >= threshold) an++;
                current -= arr[index - k];
            }
        }
        return an;
    }
};
```
