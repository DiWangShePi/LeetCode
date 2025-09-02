# 464. Can I Win

### Description

In the "100 game" two players take turns adding, to a running total, any integer from 1 to 10. The player who first causes the running total to reach or exceed 100 wins.

What if we change the game so that players cannot re-use integers?

For example, two players might take turns drawing from a common pool of numbers from 1 to 15 without replacement until they reach a total >= 100.

Given two integers maxChoosableInteger and desiredTotal, return true if the first player to move can force a win, otherwise, return false. Assume both players play optimally.

### Example 

###### Example I

> Input: maxChoosableInteger = 10, desiredTotal = 11
> Output: false
> Explanation:
> No matter which integer the first player choose, the first player will lose.
> The first player can choose an integer from 1 up to 10.
> If the first player choose 1, the second player can only choose integers from 2 up to 10.
> The second player will win by choosing 10 and get a total = 11, which is >= desiredTotal.
> Same with other integers chosen by the first player, the second player will always win.

###### Example II

> Input: maxChoosableInteger = 10, desiredTotal = 0
> Output: true

###### Example III

> Input: maxChoosableInteger = 10, desiredTotal = 1
> Output: true

### Solution

我觉得这其实就是一种暴力，但是usedNumber这个用一个数来表征用过的数字的方式十分优雅，我很欣赏

```c++
class Solution {
public:
    unordered_map<int, bool> memo;

    bool canIWin(int maxChoosableInteger, int desiredTotal) {
        if ((1 + maxChoosableInteger) * (maxChoosableInteger) / 2 < desiredTotal) {
            return false;
        }
        return dfs(maxChoosableInteger, 0, desiredTotal, 0);
    }

    bool dfs(int maxChoosableInteger, int usedNumbers, int desiredTotal, int currentTotal) {
        if (!memo.count(usedNumbers)) {
            bool res = false;
            for (int i = 0; i < maxChoosableInteger; i++) {
                if (((usedNumbers >> i) & 1) == 0) {
                    if (i + 1 + currentTotal >= desiredTotal) {
                        res = true;
                        break;
                    }
                    if (!dfs(maxChoosableInteger, usedNumbers | (1 << i), desiredTotal, currentTotal + i + 1)) {
                        res = true;
                        break;
                    }
                }
            }
            memo[usedNumbers] = res;
        }
        return memo[usedNumbers];
    }
};
```
