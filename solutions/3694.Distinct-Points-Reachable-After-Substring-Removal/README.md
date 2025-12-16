# 3694. Distinct Points Reachable After Substring Removal

**Tags:** Sliding Window, Dict

### Description

You are given a string s consisting of characters 'U', 'D', 'L', and 'R', representing moves on an infinite 2D Cartesian grid.

- 'U': Move from (x, y) to (x, y + 1).
- 'D': Move from (x, y) to (x, y - 1).
- 'L': Move from (x, y) to (x - 1, y).
- 'R': Move from (x, y) to (x + 1, y).
You are also given a positive integer k.

You must choose and remove exactly one contiguous substring of length k from s. Then, start from coordinate (0, 0) and perform the remaining moves in order.

Return an integer denoting the number of distinct final coordinates reachable.

### Example

###### Example I

> Input: s = "LUL", k = 1
> Output: 2
> Explanation:
> After removing a substring of length 1, s can be "UL", "LL" or "LU". Following these moves, the final coordinates will be (-1, 1), (-2, 0) and (-1, 1) respectively. There are two distinct points (-1, 1) and (-2, 0) so the answer is 2.

###### Example II

> Input: s = "UDLR", k = 4
> Output: 1
> Explanation:
> After removing a substring of length 4, s can only be the empty string. The final coordinates will be (0, 0). There is only one distinct point (0, 0) so the answer is 1.

###### Example III

> Input: s = "UU", k = 1
> Output: 1
> Explanation:
> After removing a substring of length 1, s becomes "U", which always ends at (0, 1), so there is only one distinct final coordinate.

### Solution

先遍历字符串走到终点，用滑动窗口在指定范围内选择撤销部分步数，用字典记录最终能达到的所有可能

```c++
class Solution {
public:
    int distinctPoints(string s, int k) {
        unordered_map<long long, int> dict;
        int x = 0, y = 0;
        for (char c : s) step(c, x, y);

        int i = 0;
        for ( ; i < k; i++) unstep(s[i], x, y);
        dict[pos(x, y)] = 1;

        for ( ; i < s.size(); i++) {
            unstep(s[i], x, y);
            step(s[i - k], x, y);
            dict[pos(x, y)] = 1;
        }
        return dict.size();
    }

private:
    void step(char c, int& x, int& y) {
        if (c == 'U') y++;
        if (c == 'D') y--;
        if (c == 'L') x--;
        if (c == 'R') x++;
    }

    void unstep(char c, int& x, int& y) {
        if (c == 'U') y--;
        if (c == 'D') y++;
        if (c == 'L') x++;
        if (c == 'R') x--;
    }

    long long pos(int x, int y) {
        return (long long) x * 1000000 + y;
    }
};
```
