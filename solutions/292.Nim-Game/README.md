# 292. Nim Game

### Description

You are playing the following Nim Game with your friend:

- Initially, there is a heap of stones on the table.
- You and your friend will alternate taking turns, and you go first.
- On each turn, the person whose turn it is will remove 1 to 3 stones from the heap.
- The one who removes the last stone is the winner.
Given n, the number of stones in the heap, return true if you can win the game assuming both you and your friend play optimally, otherwise return false.

### Example 

###### Example I:

```
Input: n = 4
Output: false
Explanation: These are the possible outcomes:
1. You remove 1 stone. Your friend removes 3 stones, including the last stone. Your friend wins.
2. You remove 2 stones. Your friend removes 2 stones, including the last stone. Your friend wins.
3. You remove 3 stones. Your friend removes the last stone. Your friend wins.
In all outcomes, your friend wins.
```

###### Example II

```
Input: n = 1
Output: true
```

###### Example II

```
Input: n = 2
Output: true
```

### Solution

我们先来考虑个数小时游戏的输赢情况：

- 当n等于1，2，3时，我们可以一次性将石头全部拿完。
- 当n等于4时，按照例子中给出的几个情况，我们一定会输。

假设初始的石头多于4个，如果我们能以拿走一部分石头以至于剩下4个（或四个的倍数），对手就会输掉，如果不能，我就会输掉。

```c++
class Solution {
public:
    bool canWinNim(int n) {
        return n % 4 != 0;
    }
};
```
