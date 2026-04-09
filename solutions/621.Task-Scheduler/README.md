# 621. Task Scheduler

**Tags:** Greedy

### Description

You are given an array of CPU tasks, each labeled with a letter from A to Z, and a number n. Each CPU interval can be idle or allow the completion of one task. Tasks can be completed in any order, but there's a constraint: there has to be a gap of at least n intervals between two tasks with the same label.

Return the minimum number of CPU intervals required to complete all tasks.

### Example

###### Example I

> Input: tasks = ["A","A","A","B","B","B"], n = 2
> Output: 8
> Explanation: A possible sequence is: A -> B -> idle -> A -> B -> idle -> A -> B.
> After completing task A, you must wait two intervals before doing A again. The same applies to task B. In the 3rd interval, neither A nor B can be done, so you idle. By the 4th interval, you can do A again as 2 intervals have passed.

###### Example II

> Input: tasks = ["A","C","A","B","D","B"], n = 1
> Output: 6
> Explanation: A possible sequence is: A -> B -> C -> D -> A -> B.
> With a cooling interval of 1, you can repeat a task after just one other task.

###### Example III

> Input: tasks = ["A","A","A", "B","B","B"], n = 3
> Output: 10
> Explanation: A possible sequence is: A -> B -> idle -> idle -> A -> B -> idle -> idle -> A -> B.
> There are only two types of tasks, A and B, which need to be separated by 3 intervals. This leads to idling twice between repetitions of these tasks.

### Solution

对于任务列表中个数最多的任务，设任务个数为 count，我们知道至少需要 (count - 1)n + 1 的时间来完成。如果存在 x 个这样的任务，我们知道至少需要 (count - 1)n + x 的时间来完成。

现在，对于其余的任务，我们可以将其填补在上面的日程规划中“空白”的地方，如果空白的不够，那总任务时间就是任务个数。

```c++
class Solution {
public:
    int leastInterval(vector<char>& tasks, int n) {
        unordered_map<int, int> dict;
        for (char t : tasks) dict[t -'A']++;

        int maxExec = 0, count = 0;
        for (int i = 0; i < 26; i++) {
            if (dict.count(i) != 0) maxExec = max(maxExec, dict[i]);
        }
        for (int i = 0; i < 26; i++) {
            if (dict.count(i) != 0 && dict[i] == maxExec) count++;
        }
        int a = (maxExec - 1) * (n + 1) + count;
        int b = tasks.size();
        return max(a, b);
    }
};
```
