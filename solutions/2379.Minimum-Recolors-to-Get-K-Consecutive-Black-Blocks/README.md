# 2379. Minimum Recolors to Get K Consecutive Black Blocks

**Tags:** Sliding Window

### Description

You are given a 0-indexed string blocks of length n, where blocks[i] is either 'W' or 'B', representing the color of the ith block. The characters 'W' and 'B' denote the colors white and black, respectively.

You are also given an integer k, which is the desired number of consecutive black blocks.

In one operation, you can recolor a white block such that it becomes a black block.

Return the minimum number of operations needed such that there is at least one occurrence of k consecutive black blocks.

### Example

###### Example I

> Input: blocks = "WBBWWBBWBW", k = 7
> Output: 3
> Explanation:
> One way to achieve 7 consecutive black blocks is to recolor the 0th, 3rd, and 4th blocks
> so that blocks = "BBBBBBBWBW". 
> It can be shown that there is no way to achieve 7 consecutive black blocks in less than 3 operations.
> Therefore, we return 3.

###### Example II

> Input: blocks = "WBWBBBW", k = 2
> Output: 0
> Explanation:
> No changes need to be made, since 2 consecutive black blocks already exist.
> Therefore, we return 0.

### Solution

滑动窗口，参见：https://leetcode.cn/discuss/post/3578981/ti-dan-hua-dong-chuang-kou-ding-chang-bu-rzz7/

```c++
class Solution {
public:
    int minimumRecolors(string blocks, int k) {
        int occur = 0, i = 0;
        for ( ; i < k; i++)
            if (blocks[i] == 'W') occur++;
        
        int an = occur;
        for ( ; i < blocks.size(); i++) {
            if (blocks[i] == 'W') occur++;
            if (blocks[i - k] == 'W') occur--;

            an = min(an, occur);
        }
        return an;
    }
};
```
