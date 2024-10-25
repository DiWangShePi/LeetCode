# 84. Largest Rectangle in Histogram

### Question Description

Given an array of integers heights representing the histogram's bar height where the width of each bar is 1, return the area of the largest rectangle in the histogram.

**Example: **

```
Input: heights = [2,1,5,6,2,3]
Output: 10
Explanation: The above is a histogram where width of each bar is 1.
The largest rectangle is shown in the red area, which has an area = 10 units.
```

```
Input: heights = [2,4]
Output: 4
```

### Question Solution

我一开始的想法是如何暴力？暴力的解法是遍历每一种可能的子序列，查询这个子序列的最小值，计算矩形大小，更新最大值。
这样的解法是O(n^3)的复杂度。

随后考虑怎么便捷的查询指定区间的最小值，使用线段树可以O(1)的解决这个问题，但有O(nlog(n))的预处理复杂度。

接下来我们发现，一个区间的值是由区间最小值决定的，而只要包含了这个最小值的子区间，其大小一定不会大于当前值。因此我们可以简化掉
一部分子区间，每次只考虑最小值左边和右边的区间，递归的计算他们的值，再返回比较。在最坏的情况下，我们依然会有O(n)的复杂度，且
考虑到前面线段树的预处理，算法的复杂度是O(nlog(n))的。

以上的思路是已知宽度，求高度再分割的思想。我们可以反过来，已知高度求宽度。对于每一个特定高度的柱子，向左边和右边扩展，寻找
第一个比这个数字小的柱子，即为当前柱子所能获得的最大宽度。这里查询如果使用遍历就会退化成O(n^2)的复杂度，我们可以在每一次获取
当前柱子的最小值的index时，将其套用到下一次查询，从而简化时间复杂度到O(n)。最后再遍历一遍柱子，记录每一个矩形大小并保留最小值。

### Code Implementation

###### c++

```c++
class Solution {
public:
    int largestRectangleArea(vector<int>& arr) {
        int n = arr.size();
        int psm[n];
        for(int i = 0; i < n; i++){
            int prev_index = i - 1;
            while(prev_index >= 0 && arr[prev_index] >= arr[i]){ 
                prev_index = psm[prev_index];
            }
            psm[i] = prev_index;
        }
        int nsm[n];
        for(int i = n - 1; i >= 0; i--){
            int next_index = i + 1;
            while(next_index < n && arr[next_index] >= arr[i]){
                next_index = nsm[next_index];
            }
            nsm[i] = next_index;
        }

        int area = 0;
        for(int i = 0; i < n; i++){
            int height = arr[i];
            int width = nsm[i] - psm[i] - 1;
            area = max(area, height * width);
        }
        return area;
    }
};
```