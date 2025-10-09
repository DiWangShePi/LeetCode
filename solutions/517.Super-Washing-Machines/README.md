# 517. Super Washing Machines

### Description

You have n super washing machines on a line. Initially, each washing machine has some dresses or is empty.

For each move, you could choose any m (1 <= m <= n) washing machines, and pass one dress of each washing machine to one of its adjacent washing machines at the same time.

Given an integer array machines representing the number of dresses in each washing machine from left to right on the line, return the minimum number of moves to make all the washing machines have the same number of dresses. If it is not possible to do it, return -1.

### Example 

###### Example I

> Input: machines = [1,0,5]
> Output: 3
> Explanation:
> 1st move:    1     0 <-- 5    =>    1     1     4
> 2nd move:    1 <-- 1 <-- 4    =>    2     1     3
> 3rd move:    2     1 <-- 3    =>    2     2     2

###### Example II

> Input: machines = [0,3,0]
> Output: 2
> Explanation:
> 1st move:    0 <-- 3     0    =>    1     2     0
> 2nd move:    1     2 --> 0    =>    1     1     1

###### Example III

> Input: machines = [0,2,0]
> Output: -1
> Explanation:
> It's impossible to make all three washing machines have the same number of dresses.

### Solution

我们的目标是让所有的洗衣机拥有同样数量的衣服，所以我们可以计算出衣服的总量，再除以洗衣机的个数，得到每个目标的衣服数量。

如果不能整除，就代表不可能实现。

接下来，我们可以得知，对于每一个洗衣机而言，他需要额外补充或者给出多少的衣服，才能达到预设的数量。

如果一件洗衣机多出预设值N件衣服，那么他就需要给出N件衣服，我们的答案不会低于这个值。洗衣机少于预设值时同理。

特别的，我们需要考虑：如果缺失衣服的洗衣机在一个连续的区间里，由于衣服要一件件的传递，此时需要的传递次数会是
缺失的衣服数量的和，而非单个缺失的最大值。

```c++
class Solution {
public:
    int findMinMoves(vector<int>& machines) {
        int sum = accumulate(machines.begin(), machines.end(), 0);
        if (sum % machines.size() != 0) return -1;
        int average = sum / machines.size();

        int an = INT_MIN;
        int current = 0;
        for (int m : machines) {
            current += m - average;
            an = max(an, max(abs(current), m - average));
        }
        return an;
    }
};
```
