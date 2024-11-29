# 118. Pascal's Triangle

### Description

Given an integer numRows, return the first numRows of Pascal's triangle.

In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:

### Solution

按要求实现即可

### Implementation

###### c++

```c++
class Solution {
public:
    vector<vector<int>> generate(int numRows) {
        vector<vector<int>> result;
        vector<int> init(1, 1);
        result.push_back(init);

        for (int i = 1; i < numRows; i++) {
            vector<int> last = result[result.size() - 1];
            vector<int> current(1, 1);
            for (int j = 1; j < i + 1; j++) {
                int num = last[j - 1];
                if (j < last.size()) num = num + last[j];
                current.push_back(num);
            }
            result.push_back(current);
        }
        return result;
    }
};
```