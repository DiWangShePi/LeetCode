# 447. Number of Boomerangs

### Description

You are given n points in the plane that are all distinct, where points[i] = [xi, yi]. A boomerang is a tuple of points (i, j, k) such that the distance between i and j equals the distance between i and k (the order of the tuple matters).

Return the number of boomerangs.

### Example 

###### Example I

> Input: points = [[0,0],[1,0],[2,0]]
> Output: 2
> Explanation: The two boomerangs are [[1,0],[0,0],[2,0]] and [[1,0],[2,0],[0,0]].

###### Example II

> Input: points = [[1,1],[2,2],[3,3]]
> Output: 2

###### Example III

> Input: points = [[1,1]]
> Output: 0

### Solution

暴力的解法：遍历计算所有的点，计算他们之间的距离。再枚举尝试每一种组合，符合要求的组合就计入答案。

```c++
class Solution {
public:
    int numberOfBoomerangs(vector<vector<int>>& points) {
        int n = points.size();
        if (n == 0) return 0;

        vector<vector<double>> dis(n, vector<double>(n, 0));
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                dis[i][j] = distance(points[i], points[j]);
            }
        }

        int an = 0;
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                if (i == j) continue;

                for (int k = 0; k < n; k++) {
                    if (i == k || j == k) continue;
                    if (dis[i][j] == dis[j][k]) an++;
                }
            }
        }
        return an;
    }

private:
    double distance(vector<int> a, vector<int> b) {
        int x = a[0] - b[0];
        int y = a[1] - b[1];
        return sqrt(x * x + y * y);
    }
};
```

我们很快发现，上述解法中，我们可以将相同距离的两个点组成的列表提出来，再在这里面检查是否符合有一个共同的点的要求

```c++
class Solution {
public:
    int numberOfBoomerangs(vector<vector<int>>& points) {
        int n = points.size();
        if (n == 0) return 0;

        unordered_map<double, vector<pair<int, int>>> dis; 
        for (int i = 0; i < n; i++) {
            for (int j = i + 1; j < n; j++) {
                double current_dis = distance(points[i], points[j]);
                dis[current_dis].push_back(make_pair(i, j));
            }
        }

        int an = 0;
        for (auto it = dis.begin(); it != dis.end(); it++) {
            vector<pair<int, int>> c = it->second;

            for (int i = 0; i < c.size(); i++) {
                for (int j = i + 1; j < c.size(); j++) {
                    if (
                        c[i].first == c[j].first 
                        ||
                        c[i].first == c[j].second
                        ||
                        c[i].second == c[j].first
                        ||
                        c[i].second == c[j].second
                    ) an += 2;
                }
            }
        }
        return an;
    }

private:
    double distance(vector<int> a, vector<int> b) {
        int x = a[0] - b[0];
        int y = a[1] - b[1];
        return sqrt(x * x + y * y);
    }
};
```

再进一步，我们可以枚举每个点，考虑所有其他的点到当前这个点的距离，随后计算排列数

```c++
class Solution {
public:
    int numberOfBoomerangs(vector<vector<int>> &points) {
        int an = 0;
        for (auto &p : points) {
            unordered_map<int, int> cnt;
            for (auto &q : points) {
                int dis = (p[0] - q[0]) * (p[0] - q[0]) + (p[1] - q[1]) * (p[1] - q[1]);
                ++cnt[dis];
            }
            for (auto &[_, m] : cnt) {
                an += m * (m - 1);
            }
        }
        return an;
    }
};
```
