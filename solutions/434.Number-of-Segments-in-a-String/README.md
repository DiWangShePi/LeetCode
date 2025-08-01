# 434. Number of Segments in a String

### Description

Given a string s, return the number of segments in the string.

A segment is defined to be a contiguous sequence of non-space characters.

### Example 

###### Example I

> Input: s = "Hello, my name is John"
> Output: 5
> Explanation: The five segments are ["Hello,", "my", "name", "is", "John"]

###### Example II

> Input: s = "Hello"
> Output: 1

### Solution

遍历数数

```c++
class Solution {
public:
    int countSegments(string s) {
        int count = 0;
        bool inSegment = false;

        for (char c : s) {
            if (c != ' ') {
                if (!inSegment) {
                    count++;
                    inSegment = true;
                }
            } else {
                inSegment = false;
            }
        }
        return count;
    }
};
```
