# 556. Next Greater Element III

**Tags:** String

### Description

Given a positive integer n, find the smallest integer which has exactly the same digits existing in the integer n and is greater in value than n. If no such positive integer exists, return -1.

Note that the returned integer should fit in 32-bit integer, if there is a valid answer but it does not fit in 32-bit integer, return -1.

### Example 

###### Example I

> Input: n = 12
> Output: 21

###### Example II

> Input: n = 21
> Output: -1

### Solution

从后往前遍历数字，对于当前数字，检查其后续数字中是否存在比它大的数字，选择满足要求的最小数字替代掉当前数字，然后重构后续数字为最小版本。

```c++
class Solution {
public:
    int nextGreaterElement(int n) {
        vector<int> dict(10, 0);
        string N = to_string(n);

        dict[N.back() - '0']++;
        
        for (int i = N.size() - 2; i > -1; i--) {
            int c = N[i] - '0';
            dict[c]++;
            for (int j = c + 1; j < 10; j++) {
                if (dict[j] > 0) {
                    string an = N.substr(0, i) + char(j + '0');
                    dict[j]--;
                    for (int k = 0; k < 10; k++) {
                        if (dict[k] != 0) {
                            an += string(dict[k], char(k + '0'));
                         }
                    }
                    if (stol(an) > INT_MAX) {
                        dict[j]++;
                        break;
                    }
                    return stoi(an);
                }
            }
        }
        return -1;
    }
};
```
