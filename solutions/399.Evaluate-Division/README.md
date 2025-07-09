# 399. Evaluate Division

### Description

You are given an array of variable pairs equations and an array of real numbers values, where equations[i] = [Ai, Bi] and values[i] represent the equation Ai / Bi = values[i]. Each Ai or Bi is a string that represents a single variable.

You are also given some queries, where queries[j] = [Cj, Dj] represents the jth query where you must find the answer for Cj / Dj = ?.

Return the answers to all queries. If a single answer cannot be determined, return -1.0.

Note: The input is always valid. You may assume that evaluating the queries will not result in division by zero and that there is no contradiction.

Note: The variables that do not occur in the list of equations are undefined, so the answer cannot be determined for them.

### Example 

###### Exampel I

> Input: equations = [["a","b"],["b","c"]], values = [2.0,3.0], queries = [["a","c"],["b","a"],["a","e"],["a","a"],["x","x"]]
> Output: [6.00000,0.50000,-1.00000,1.00000,-1.00000]
> Explanation: 
> Given: a / b = 2.0, b / c = 3.0
> queries are: a / c = ?, b / a = ?, a / e = ?, a / a = ?, x / x = ? 
> return: [6.0, 0.5, -1.0, 1.0, -1.0 ]
> note: x is undefined => -1.0

###### Example II

> Input: equations = [["a","b"],["b","c"],["bc","cd"]], values = [1.5,2.5,5.0], queries = [["a","c"],["c","b"],["bc","cd"],["cd","bc"]]
> Output: [3.75000,0.40000,5.00000,0.20000]

###### Example III

> Input: equations = [["a","b"]], values = [0.5], queries = [["a","b"],["b","a"],["a","c"],["x","y"]]
> Output: [0.50000,2.00000,-1.00000,-1.00000]

### Solution

根据给定的equation和values，我们可以构建图，记录每一个字符串的邻居和权重。
对于给定的查询，在图中寻找对应的值，在找到时返回路径累计计算得到的权值。

```c++
class Solution {
public:
    vector<double> calcEquation(vector<vector<string>>& equations, vector<double>& values, vector<vector<string>>& queries) {
        unordered_map<string, unordered_map<string, double>> graph;
        for (int i = 0; i < equations.size(); ++i) {
            string u = equations[i][0];
            string v = equations[i][1];
            double val = values[i];
            graph[u][v] = val;       // u / v = val
            graph[v][u] = 1.0 / val; // v / u = 1 / val
        }

        vector<double> results;
        for (const auto& query : queries) {
            string x = query[0];
            string y = query[1];
            if (graph.find(x) == graph.end() || graph.find(y) == graph.end()) {
                results.push_back(-1.0);
            } else if (x == y) {
                results.push_back(1.0);
            } else {
                unordered_set<string> visited;
                double res = dfs(x, y, graph, visited);
                results.push_back(res);
            }
        }
        return results;
    }

private:
    double dfs(const string& src, const string& dst, 
               unordered_map<string, unordered_map<string, double>>& graph, 
               unordered_set<string>& visited) {
        if (src == dst) return 1.0;
        visited.insert(src);
        for (const auto& neighbor : graph[src]) {
            string next = neighbor.first;
            double weight = neighbor.second;
            if (visited.find(next) == visited.end()) {
                double res = dfs(next, dst, graph, visited);
                if (res != -1.0) {
                    return weight * res;
                }
            }
        }
        return -1.0;
    }
};
```
