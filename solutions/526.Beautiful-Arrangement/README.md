# 526. Beautiful Arrangement

**Tags:** DFS

### Description

Suppose you have n integers labeled 1 through n. A permutation of those n integers perm (1-indexed) is considered a beautiful arrangement if for every i (1 <= i <= n), either of the following is true:

- perm[i] is divisible by i.
- i is divisible by perm[i].
Given an integer n, return the number of the beautiful arrangements that you can construct.

### Example 

###### Example I

> Input: n = 2
> Output: 2
> Explanation: 
> The first beautiful arrangement is [1,2]:
>     - perm[1] = 1 is divisible by i = 1
>     - perm[2] = 2 is divisible by i = 2
> The second beautiful arrangement is [2,1]:
>     - perm[1] = 2 is divisible by i = 1
>     - i = 2 is divisible by perm[2] = 1

###### Example II

> Input: n = 1
> Output: 1

### Solution

回溯是一种暴力：

```c++
class Solution {
public:
    int countArrangement(int n) {
        vector<vector<int>> match(n + 1);
        vector<bool> visited(n + 1, false);
        for (int i = 1; i <= n; i++) {
            for (int j = 1; j <= n; j++) {
                if (i % j == 0 || j % i == 0)
                    match[i].push_back(j);
            }
        }

        int an = 0;
        dfs(match, visited, 1, an);
        return an;
    }

private:
    void dfs(vector<vector<int>>& match, vector<bool>& visited, int index, int& an) {
        if (index == visited.size()) {
            an++;
            return;
        }

        for (int v : match[index]) {
            if (!visited[v]) {
                visited[v] = true;
                dfs(match, visited, index + 1, an);
                visited[v] = false;
            }
        }
    }
};
```

查表也是一种暴力：

```c++
class Solution {
public:
    int countArrangement(int n) {
        vector<int> an{0,1,2,3,8,10,36,41,132,250,700,750,4010,4237,10680,24679};
        return an[n];
    }
};
```
