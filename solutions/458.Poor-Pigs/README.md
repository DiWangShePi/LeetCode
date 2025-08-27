# 458. Poor Pigs

###### Description

There are buckets buckets of liquid, where exactly one of the buckets is poisonous. To figure out which one is poisonous, you feed some number of (poor) pigs the liquid to see whether they will die or not. Unfortunately, you only have minutesToTest minutes to determine which bucket is poisonous.

You can feed the pigs according to these steps:

1. Choose some live pigs to feed.
2. For each pig, choose which buckets to feed it. The pig will consume all the chosen buckets simultaneously and will take no time. Each pig can feed from any number of buckets, and each bucket can be fed from by any number of pigs.
3. Wait for minutesToDie minutes. You may not feed any other pigs during this time.
4. After minutesToDie minutes have passed, any pigs that have been fed the poisonous bucket will die, and all others will survive.
5. Repeat this process until you run out of time.
Given buckets, minutesToDie, and minutesToTest, return the minimum number of pigs needed to figure out which bucket is poisonous within the allotted time.

### Example 

###### Example I

> Input: buckets = 4, minutesToDie = 15, minutesToTest = 15
> Output: 2
> Explanation: We can determine the poisonous bucket as follows:
> At time 0, feed the first pig buckets 1 and 2, and feed the second pig buckets 2 and 3.
> At time 15, there are 4 possible outcomes:
> - If only the first pig dies, then bucket 1 must be poisonous.
> - If only the second pig dies, then bucket 3 must be poisonous.
> - If both pigs die, then bucket 2 must be poisonous.
> - If neither pig dies, then bucket 4 must be poisonous.

###### Example II

> Input: buckets = 4, minutesToDie = 15, minutesToTest = 30
> Output: 2
> Explanation: We can determine the poisonous bucket as follows:
> At time 0, feed the first pig bucket 1, and feed the second pig bucket 2.
> At time 15, there are 2 possible outcomes:
> - If either pig dies, then the poisonous bucket is the one it was fed.
> - If neither pig dies, then feed the first pig bucket 3, and feed the second pig bucket 4.
> At time 30, one of the two pigs must die, and the poisonous bucket is the one it was fed.

### Solution

> 经过所有实验，一只小猪可能有多少状态？第一轮死、第二轮死、...、第n轮死，和最后生还。共有n+1种状态。
> n+1种状态所携带的信息共log(n+1)个比特，这也是一个小猪能携带的最多信息。
> 哪一个桶包含毒药存在buckets个可能性，携带的信息为log(buckets)。
> 因此一定存在一种实验设计方法，使得当我们有K只猪，且满足K*log(n + 1) >= log(buckets)时，我们就一定能得到足够的信息去判断哪个桶是有毒的。

参考： https://leetcode.cn/problems/poor-pigs/solutions/1119222/ke-lian-de-xiao-zhu-by-leetcode-solution-z0h7/

```c++
class Solution {
public:
    int poorPigs(int buckets, int minutesToDie, int minutesToTest) {
        int states = minutesToTest / minutesToDie + 1;
        int pigs = ceil(log(buckets) / log(states) - 1e-5);
        return pigs;
    }
};
```
