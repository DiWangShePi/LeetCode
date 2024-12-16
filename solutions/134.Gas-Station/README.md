# 134. Gas Station

### Description

There are n gas stations along a circular route, where the amount of gas at the ith station is gas[i].

You have a car with an unlimited gas tank and it costs cost[i] of gas to travel from the ith station to its next (i + 1)th station. You begin the journey with an empty tank at one of the gas stations.

Given two integer arrays gas and cost, return the starting gas station's index if you can travel around the circuit once in the clockwise direction, otherwise return -1. If there exists a solution, it is guaranteed to be unique.

### Solution

最简单的想法，暴力。尝试每一个可能的开始。

我们可以轻易的发现，暴力算法在开始的时候就可以被简化。
介于我们在开始的时候没有gas，如果当前站的cost大于gas，那么我们就不可能从这里出发（刚开始就会夭折）。
因此，我们可以将gas和cost列表相减，得到一个新的列表。我们只会考虑从这个列表中大于等于0的元素开始。

我们的问题变成了：对于这个新的列表，从哪一个元素出发，可以返回到开始的位置，且途中不会小于0。
我们可以从任意一个大于0的数字开始，且如果中途失败了（累加和小于0），则可以直接从下一个大于0的元素开始（因为显然这中间的任何一个元素都不会能满足条件）。

### Implementation

###### c++

```c++
class Solution {
public:
    int canCompleteCircuit(vector<int>& gas, vector<int>& cost) {
        int c = 0, f = 0, r = -1, ex = 0;
        for (int i = 0; i < gas.size(); i++)
        {
            f += gas[i];
            c += cost[i];

            if (gas[i] + ex >= cost[i])
            {
                ex += gas[i] - cost[i];
                if (r == -1)
                r = i;
            } else {
                ex = 0;
                r = -1;
            }
        }
        if(f < c) return -1;
        return r;
    }
};
```
