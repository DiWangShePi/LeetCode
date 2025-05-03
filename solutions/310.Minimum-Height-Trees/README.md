# 310. Minimum Height Trees

### Description

A tree is an undirected graph in which any two vertices are connected by exactly one path. In other words, any connected graph without simple cycles is a tree.

Given a tree of n nodes labelled from 0 to n - 1, and an array of n - 1 edges where edges[i] = [ai, bi] indicates that there is an undirected edge between the two nodes ai and bi in the tree, you can choose any node of the tree as the root. When you select a node x as the root, the result tree has height h. Among all possible rooted trees, those with minimum height (i.e. min(h))  are called minimum height trees (MHTs).

Return a list of all MHTs' root labels. You can return the answer in any order.

The height of a rooted tree is the number of edges on the longest downward path between the root and a leaf.

### Example 

###### Example I

![](./e1.jpg)

```
Input: n = 4, edges = [[1,0],[1,2],[1,3]]
Output: [1]
Explanation: As shown, the height of the tree is 1 when the root is the node with label 1 which is the only MHT.
```

###### Example II

![](./e2.jpg)

```
Input: n = 6, edges = [[3,0],[3,1],[3,2],[3,4],[5,4]]
Output: [3,4]
```

### Solution

先想着暴力，然后超时了

> 但是这一次，我却让广度优先搜索丢了脸

```c++
class Solution {
public:
    vector<int> findMinHeightTrees(int n, vector<vector<int>>& edges) {
        vector<vector<int>> adj(n);
        for (vector<int> edge : edges) {
            adj[edge[0]].push_back(edge[1]);
            adj[edge[1]].push_back(edge[0]);
        }

        vector<int> an;
        int min_height = n;
        for (int i = 0; i < n; i++) {
            int height = bfs(n, i, adj);

            if (min_height > height) {
                an.clear();
                an.push_back(i);

                min_height = height;
            } else if (min_height == height) an.push_back(i);
        }
        return an;
    }

private:
    int bfs(int n, int root, const vector<vector<int>>& adj) {
        queue<int> q;
        q.push(root);
        vector<bool> visited(n, false);
        visited[root] = true;

        int count = 0;
        while (!q.empty()) {
            int level_size = q.size();

            for (int i = 0; i < level_size; i++) {
                int current = q.front();
                q.pop();

                for (int edge : adj[current]) {
                    if (!visited[edge]) {
                        q.push(edge);
                        visited[edge] = true;
                    }
                }
            }

            if (!q.empty()) count++;
        }
        return count;
    }
};
```

换用拓扑排序，从外向内一点点的剥离，直到只剩下一个或两个节点

```c++
class Solution {
public:
    vector<int> findMinHeightTrees(int n, vector<vector<int>>& edges) {
        if (n == 1) return {0};

        // create adj and drgree
        vector<int> degree(n, 0);
        vector<vector<int>> adj(n);
        for (const auto & edge : edges) {
            adj[edge[0]].emplace_back(edge[1]);
            adj[edge[1]].emplace_back(edge[0]);
            degree[edge[0]]++;
            degree[edge[1]]++;
        }

        queue<int> qu;
        vector<int> ans;
        for (int i = 0; i < n; i++) if (degree[i] == 1) qu.emplace(i);
        int remain = n;
        while (remain > 2) {
            int sz = qu.size();
            remain -= sz;
            for (int i = 0; i < sz; i++) {
                int curr = qu.front();
                qu.pop();

                for (const auto & v : adj[curr]) {
                    if (--degree[v] == 1) {
                        qu.emplace(v);
                    }
                }
            }
        }

        while (!qu.empty()) {
            ans.emplace_back(qu.front());
            qu.pop();
        }
        return ans;
    }
};
```

第三种方式是两遍遍历
- 第一遍随机开始找最远点
- 第二遍从上一步的最远点开始找另一个最远点

两个最远点构成的路径上中间的节点（或者两个节点）即为答案。

> 我们将树看作是一种特殊的无向图，其中任意两个节点之间都有唯一一条路径。所谓“最小高度树”的根，其实就是让整棵树中从根到最远叶子节点的路径尽可能短。也就是说，我们希望根节点到其它所有节点的最远距离最小。
> 在一棵树中，这样的根节点恰好出现在树的直径的中心位置。树的直径，是指树中任意两点之间最长路径的那一对端点之间的路径。假设我们找到了这条最长路径，那么路径上的中间节点就是所有节点中距离最远节点“最居中”的一个点。这个点作为根节点时，到最远叶子节点的距离也最小，因而可以得到最小高度的树。
>实现上，我们可以通过两次广度优先搜索（BFS）来找到这条直径路径。第一次从任意节点出发，找到距离最远的节点 A；第二次从 A 出发，再找到距离它最远的节点 B。路径 A 到 B 就是这棵树的直径，而路径的中间节点就是理想的根节点。如果路径长度是奇数，则中心是唯一的；如果是偶数，则会有两个相邻的中心节点，两个都可以作为根。
>因此，通过找到树的直径，并确定其中心节点，我们就能在所有可能的根节点中，选出能构造出最小高度树的最佳选择。 --- by chatgpt-o4

```c++
class Solution {
public:
    vector<int> findMinHeightTrees(int n, vector<vector<int>>& edges) {
        if (n == 1) return {0};

        // 建图
        vector<vector<int>> adj(n);
        for (const auto& edge : edges) {
            adj[edge[0]].emplace_back(edge[1]);
            adj[edge[1]].emplace_back(edge[0]);
        }

        // 第一次 DFS 找到最远节点
        int farNode = dfsFindFarthest(0, -1, adj).second;

        // 第二次 DFS 找直径路径
        vector<int> path;
        vector<bool> visited(n, false);
        dfsPath(farNode, -1, adj, path, visited);

        // 取路径中心节点
        int m = path.size();
        if (m % 2 == 1) {
            return {path[m / 2]};
        } else {
            return {path[m / 2 - 1], path[m / 2]};
        }
    }

private:
    // DFS 返回最大深度和对应的节点
    pair<int, int> dfsFindFarthest(int node, int parent, const vector<vector<int>>& adj) {
        pair<int, int> res = {0, node};
        for (int nei : adj[node]) {
            if (nei == parent) continue;
            auto sub = dfsFindFarthest(nei, node, adj);
            sub.first += 1;
            if (sub.first > res.first) res = sub;
        }
        return res;
    }

    // DFS 路径恢复：从 start 到直径另一端，构造路径
    bool dfsPath(int node, int parent, const vector<vector<int>>& adj,
                 vector<int>& path, vector<bool>& visited) {
        path.push_back(node);
        visited[node] = true;
        if (adj[node].size() == 1 && node != parent) return true;  // 到达叶子
        for (int nei : adj[node]) {
            if (nei == parent || visited[nei]) continue;
            if (dfsPath(nei, node, adj, path, visited)) return true;
        }
        path.pop_back();
        return false;
    }
};
```

> 我热爱深度优先搜索...