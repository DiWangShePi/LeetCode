# 492. Construct the Rectangle

**Tags:** Math

### Description

A web developer needs to know how to design a web page's size. So, given a specific rectangular web page’s area, your job by now is to design a rectangular web page, whose length L and width W satisfy the following requirements:

1. The area of the rectangular web page you designed must equal to the given target area.
2. The width W should not be larger than the length L, which means L >= W.
3. The difference between length L and width W should be as small as possible.

Return an array [L, W] where L and W are the length and width of the web page you designed in sequence.

### Example 

###### Example I

> Input: area = 4
> Output: [2,2]
> Explanation: The target area is 4, and all the possible ways to construct it are [1,4], [2,2], [4,1]. 
> But according to requirement 2, [1,4] is illegal; according to requirement 3,  [4,1] is not optimal compared to [2,2]. So the length L is 2, and the width W is 2.

###### Example II

> Input: area = 37
> Output: [37,1]

###### Example III

> Input: area = 122122
> Output: [427,286]

### Solution

开根号，然后遍历尝试长度/宽度。

```c++
class Solution {
public:
    vector<int> constructRectangle(int area) {
        int possible_L = ceil(sqrt(area)), possible_W;
        while (possible_L < area) {
            possible_W = floor(area / possible_L);
            if (possible_W * possible_L == area) return {possible_L, possible_W};

            possible_L++;
        }
        return {area, 1};
    }
};
```

显然，当 area 比较大时，求宽度会比求长度更快

```c++
class Solution {
public:
    vector<int> constructRectangle(int area) {
        int width = static_cast<int>(sqrt(area));
        while (area % width != 0) {
            width--;
        }
        int length = area / width;
        return {length, width};
    }
};
```
