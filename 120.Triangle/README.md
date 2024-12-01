# 120. Triangle

### Description

Given a triangle array, return the minimum path sum from top to bottom.

For each step, you may move to an adjacent number of the row below. More formally, if you are on index i on the current row, you may move to either index i or index i + 1 on the next row.

### Solution

类似寻路算法，对于每一层，将该点权重更新为上一层最小的开销加本地开销，返回最后一层最小的一个。

### Implementation

###### c++

```c++
class Solution {
public:
    int minimumTotal(vector<vector<int>>& triangle) {
        vector<int> last = triangle[0];
        vector<int> current;
        for (int i = 1; i < triangle.size(); i++) {
            current = triangle[i];

            current[0] += last[0];
            current[current.size() - 1] += last[last.size() - 1];

            for (int j = 1; j < current.size() - 1; j++) {
                int minCost = min(last[j - 1], last[j]);
                current[j] += minCost;
            }
            last = current;
        }

        int result = INT_MAX;
        for (int i = 0; i < last.size(); i++) {
            result = result > last[i] ? last[i] : result;
        }
        return result;
    }
};
```
