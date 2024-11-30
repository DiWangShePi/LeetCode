# 119. Pascal's Triangle II

### Description

Given an integer rowIndex, return the rowIndexth (0-indexed) row of the Pascal's triangle.

In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:

### Solution

与上一题一样

### Implementation

###### c++

```c++
class Solution {
public:
    vector<int> getRow(int rowIndex) {
        vector<int> last;
        vector<int> result(1, 1);

        for (int i = 1; i <= rowIndex; i++) {
            last = result;
            result.clear(); 
            for (int j = 0; j <= i; j++) {
                int num = (j > 0 ? last[j - 1] : 0) + (j < last.size() ? last[j] : 0);
                result.push_back(num);
            }
        }
        return result;
    }
};
```