# 502. IPO

**Tags:** Greedy Algorithm, Heap

### Description

Suppose LeetCode will start its IPO soon. In order to sell a good price of its shares to Venture Capital, LeetCode would like to work on some projects to increase its capital before the IPO. Since it has limited resources, it can only finish at most k distinct projects before the IPO. Help LeetCode design the best way to maximize its total capital after finishing at most k distinct projects.

You are given n projects where the ith project has a pure profit profits[i] and a minimum capital of capital[i] is needed to start it.

Initially, you have w capital. When you finish a project, you will obtain its pure profit and the profit will be added to your total capital.

Pick a list of at most k distinct projects from given projects to maximize your final capital, and return the final maximized capital.

The answer is guaranteed to fit in a 32-bit signed integer.

### Example 

###### Example I

> Input: k = 2, w = 0, profits = [1,2,3], capital = [0,1,1]
> Output: 4
> Explanation: Since your initial capital is 0, you can only start the project indexed 0.
> After finishing it you will obtain profit 1 and your capital becomes 1.
> With capital 1, you can either start the project indexed 1 or the project indexed 2.
> Since you can choose at most 2 projects, you need to finish the project indexed 2 to get the maximum capital.
> Therefore, output the final maximized capital, which is 0 + 1 + 3 = 4.

###### Example II

> Input: k = 3, w = 0, profits = [1,2,3], capital = [0,1,2]
> Output: 6

### Solution

如果没有 k 的限制，这一题的做法应该是将 capital 排序，然后一个一个做，直到做完或者不够为止。

有了 k 的限制，我们就需要一点贪心的做法，比如在能做的范围内，先挑利润丰厚的做。

> 这一题没有次序的限制，可以回过头来做，所以不适合用动态规划

```c++
class Solution {
public:
    int findMaximizedCapital(int k, int w, vector<int>& profits, vector<int>& capital) {
        int an = 0;
        priority_queue<int, vector<int>, less<int>> pq;
        vector<pair<int, int>> q;

        for (int i = 0; i < profits.size(); i++) {
            q.push_back({capital[i], profits[i]});
        }
        sort(q.begin(), q.end());

        int index = 0;
        for (int i = 0; i < k; i++) {
            while (index < profits.size() && q[index].first <= w) pq.push(q[index++].second);

            if (!pq.empty()) {
                w += pq.top();
                pq.pop();
            } else break;
        }
        return w;
    }
};
```
