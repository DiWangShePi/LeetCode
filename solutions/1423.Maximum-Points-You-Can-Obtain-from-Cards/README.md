# 1423. Maximum Points You Can Obtain from Cards

**Tags:** Sliding Window

### Description

There are several cards arranged in a row, and each card has an associated number of points. The points are given in the integer array cardPoints.

In one step, you can take one card from the beginning or from the end of the row. You have to take exactly k cards.

Your score is the sum of the points of the cards you have taken.

Given the integer array cardPoints and the integer k, return the maximum score you can obtain.

### Example

###### Example I

> Input: cardPoints = [1,2,3,4,5,6,1], k = 3
> Output: 12
> Explanation: After the first step, your score will always be 1. However, choosing the rightmost card first will maximize your total score. The optimal strategy is to take the three cards on the right, giving a final score of 1 + 6 + 5 = 12.

###### Example II

> Input: cardPoints = [2,2,2], k = 2
> Output: 4
> Explanation: Regardless of which two cards you take, your score will always be 4.

###### Example III

> Input: cardPoints = [9,7,7,9,7,7,9], k = 7
> Output: 55
> Explanation: You have to take all the cards. Your score is the sum of points of all cards.

### Solution

滑动窗口，参见：https://leetcode.cn/discuss/post/3578981/ti-dan-hua-dong-chuang-kou-ding-chang-bu-rzz7/

```c++
class Solution {
public:
    int maxScore(vector<int>& cardPoints, int k) {
        int sum = accumulate(cardPoints.begin(), cardPoints.end(), 0);
        int current = 0, length = cardPoints.size() - k;

        int i = 0;
        for ( ; i < length; i++) {
            current += cardPoints[i];
        }
        int an = sum - current;
        for ( ; i < cardPoints.size(); i++) {
            current += cardPoints[i];
            current -= cardPoints[i - length];

            an = max(an, sum - current);
        }
        return an;
    }
};
```
