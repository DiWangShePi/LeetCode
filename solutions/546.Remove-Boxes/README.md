# 546. Remove Boxes

**Tags:** Dynamic Progamming, DO IT AGAIN

### Description

You are given several boxes with different colors represented by different positive numbers.

You may experience several rounds to remove boxes until there is no box left. Each time you can choose some continuous boxes with the same color (i.e., composed of k boxes, k >= 1), remove them and get k * k points.

Return the maximum points you can get.

### Example 

###### Example I

> Input: boxes = [1,3,2,2,2,3,4,3,1]
> Output: 23
> Explanation:
> [1, 3, 2, 2, 2, 3, 4, 3, 1] 
> ----> [1, 3, 3, 4, 3, 1] (3*3=9 points) 
> ----> [1, 3, 3, 3, 1] (1*1=1 points) 
> ----> [1, 1] (3*3=9 points) 
> ----> [] (2*2=4 points)

###### Example II

> Input: boxes = [1,1,1]
> Output: 9

###### Example III

> Input: boxes = [1]
> Output: 1

### Solution

参考：https://leetcode.cn/problems/remove-boxes/solutions/374792/yi-chu-he-zi-by-leetcode-solution/

> 我感觉动态规划的核心在于如何定义状态，有了状态之后在定义状态转换

```c++
class Solution {
public:
    int dp[100][100][100];

    int removeBoxes(vector<int>& boxes) {
        memset(dp, 0, sizeof dp);
        return calculatePoints(boxes, 0, boxes.size() - 1, 0);
    }

    int calculatePoints(vector<int>& boxes, int l, int r, int k) {
        if (l > r) {
            return 0;
        }
        if (dp[l][r][k] == 0) {
            int r1 = r, k1 = k;
            while (r1 > l && boxes[r1] == boxes[r1 - 1]) {
                r1--;
                k1++;
            }
            dp[l][r][k] = calculatePoints(boxes, l, r1 - 1, 0) + (k1 + 1) * (k1 + 1);
            for (int i = l; i < r1; i++) {
                if (boxes[i] == boxes[r1]) {
                    dp[l][r][k] = max(dp[l][r][k], calculatePoints(boxes, l, i, k1 + 1) + calculatePoints(boxes, i + 1, r1 - 1, 0));
                }
            }
        }
        return dp[l][r][k];
    }
};
```
