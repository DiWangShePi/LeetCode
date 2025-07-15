# 406. Queue Reconstruction by Height

### Description

You are given an array of people, people, which are the attributes of some people in a queue (not necessarily in order). Each people[i] = [hi, ki] represents the ith person of height hi with exactly ki other people in front who have a height greater than or equal to hi.

Reconstruct and return the queue that is represented by the input array people. The returned queue should be formatted as an array queue, where queue[j] = [hj, kj] is the attributes of the jth person in the queue (queue[0] is the person at the front of the queue).

### Example

###### Example I

> Input: people = [[7,0],[4,4],[7,1],[5,0],[6,1],[5,2]]
> Output: [[5,0],[7,0],[5,2],[6,1],[4,4],[7,1]]
> Explanation:
> Person 0 has height 5 with no other people taller or the same height in front.
> Person 1 has height 7 with no other people taller or the same height in front.
> Person 2 has height 5 with two persons taller or the same height in front, which is person 0 and 1.
> Person 3 has height 6 with one person taller or the same height in front, which is person 1.
> Person 4 has height 4 with four people taller or the same height in front, which are people 0, 1, 2, and 3.
> Person 5 has height 7 with one person taller or the same height in front, which is person 1.
> Hence [[5,0],[7,0],[5,2],[6,1],[4,4],[7,1]] is the reconstructed queue.

###### Example II

> Input: people = [[6,0],[5,0],[4,0],[3,2],[2,2],[1,4]]
> Output: [[4,0],[5,0],[2,2],[3,2],[1,4],[6,0]]

### Solution

我们将原数组按第一个数字降序，第二个数字升序的方式排序一遍。再逐个插入。

由于第一个数字是从高到低遍历的，当我们遇到第i个数字时，大于等于它的一定都已经放好了，我们可以根据第二个数字直接锁定它应当处于的位置。
小于它的都在后面，不需要在意。

```c++
class Solution {
public:
    vector<vector<int>> reconstructQueue(vector<vector<int>>& people) {
        sort(people.begin(), people.end(), [](const vector<int>& u, const vector<int>& v) {
            return u[0] > v[0] || (u[0] == v[0] && u[1] < v[1]);
        });

        for (const vector<int>& p : people) {
            an.insert(an.begin() + p[1], p);
        }
        return an;
    }
};
```
