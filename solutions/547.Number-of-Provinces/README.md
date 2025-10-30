# 547. Number of Provinces

### Description

There are n cities. Some of them are connected, while some are not. If city a is connected directly with city b, and city b is connected directly with city c, then city a is connected indirectly with city c.

A province is a group of directly or indirectly connected cities and no other cities outside of the group.

You are given an n x n matrix isConnected where isConnected[i][j] = 1 if the ith city and the jth city are directly connected, and isConnected[i][j] = 0 otherwise.

Return the total number of provinces.

### Example

###### Example I

![](./graph1.jpg)

> Input: isConnected = [[1,1,0],[1,1,0],[0,0,1]]
> Output: 2

###### Example II

![](./graph2.jpg)

> Input: isConnected = [[1,0,0],[0,1,0],[0,0,1]]
> Output: 3

### Solution

广度优先搜索，或者深度优先搜索。这里的节点是双向联通的，所以不需要一开始找入分量为 0 的节点。

```c++
class Solution {
public:
    int findCircleNum(vector<vector<int>>& isConnected) {
        int n = isConnected.size();
        vector<bool> marked(n, false);
        int an = 0;
        for (int i = 0; i < n; i++) {
            if (!marked[i]) {
                marked[i] = true;
                bfs(isConnected, marked, i);
                an++;
            }
        }
        return an;
    }

private:
    void bfs(vector<vector<int>>& isConnected, vector<bool>& marked, int start) {
        vector<int> queue{start};
        int l = 0, r = 1;
        while (l < r) {
            for (int i = l; i < r; i++) {
                vector<int> current = isConnected[queue[i]];
                for (int j = 0; j < current.size(); j++) {
                    if (current[j] == 1 && !marked[j]) {
                        marked[j] = true;
                        queue.push_back(j);
                    }
                }
            }

            l = r;
            r = queue.size();
        }
    }
};
```
