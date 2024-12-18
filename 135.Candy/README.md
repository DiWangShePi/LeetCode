# 135. Candy

### Description

There are n children standing in a line. Each child is assigned a rating value given in the integer array ratings.

You are giving candies to these children subjected to the following requirements:

- Each child must have at least one candy.
- Children with a higher rating get more candies than their neighbors.
Return the minimum number of candies you need to have to distribute the candies to the children.

### Solution

我们可以将 "Children with a higher rating get more candies than their neighbors" 这条规则拆分成两部分：
- 当rating[i-1] < rating[i] 时，i的糖果数量需要比i-1号孩子的糖果数量多
- 当rating[i] > rating[i+1] 时，i的糖果数量需要比i+1号孩子的糖果数量多

遍历该数组两次，处理每个学生分别满足两条规则时，需要被分到的最小数量，两者的最大值即为最终糖果数量。

具体而言，以左遍历为例，若遍历到位置i有rating[i-1] < rating[i] 则令 left[i] = left[i-1] + 1，否则，left[i] = 1。

而右遍历时，我们不再需要额外保存一个数组，仅需要单个值即可。

### Implementation

###### c++

```c++
class Solution {
public:
    int candy(vector<int>& ratings) {
        int n = ratings.size();
        vector<int> left(n);
        for (int i = 0; i < n; i++) {
            if (i > 0 && ratings[i] > ratings[i - 1]) {
                left[i] = left[i - 1] + 1;
            } else {
                left[i] = 1;
            }
        }
        int right = 0, ret = 0;
        for (int i = n - 1; i >= 0; i--) {
            if (i < n - 1 && ratings[i] > ratings[i + 1]) {
                right++;
            } else {
                right = 1;
            }
            ret += max(left[i], right);
        }
        return ret;
    }
};
```
