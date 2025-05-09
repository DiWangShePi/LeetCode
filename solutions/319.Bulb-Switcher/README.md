# 319. Bulb Switcher

### Description

There are n bulbs that are initially off. You first turn on all the bulbs, then you turn off every second bulb.

On the third round, you toggle every third bulb (turning on if it's off or turning off if it's on). For the ith round, you toggle every i bulb. For the nth round, you only toggle the last bulb.

Return the number of bulbs that are on after n rounds.

### Example 

###### Example I

![](./bulb.jpg)

```
Input: n = 3
Output: 1
Explanation: At first, the three bulbs are [off, off, off].
After the first round, the three bulbs are [on, on, on].
After the second round, the three bulbs are [on, off, on].
After the third round, the three bulbs are [on, off, off]. 
So you should return 1 because there is only one bulb is on.
```

### Solution

假设从1到n进行遍历，如果当前数字x的一个因子是i，那么x所代表的灯泡就会进行状态转换。

因此，对于一个给定的数字X，决定X最终状态的，是它的因子数量的个数。起始状态为熄灭，如果因子数量为奇数个，那么该数字的最终状态即为亮起。偶数个即为熄灭。

但对于一个数字而言，它的因子一般是成对出现的。只有完全平方数除外。这一题由此变为给定N，找小于N的完全平方数个数。

```c++
class Solution {
public:
    int bulbSwitch(int n) {
        return static_cast<int>(sqrt(n));
    }
};
```
