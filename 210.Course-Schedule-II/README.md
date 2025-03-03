# 210. Course Schedule II

### Description

There are a total of numCourses courses you have to take, labeled from 0 to numCourses - 1. You are given an array prerequisites where prerequisites[i] = [ai, bi] indicates that you must take course bi first if you want to take course ai.

- For example, the pair [0, 1], indicates that to take course 0 you have to first take course 1.
Return the ordering of courses you should take to finish all courses. If there are many valid answers, return any of them. If it is impossible to finish all courses, return an empty array.

### Solution

转换为图，检查是否有环

### Implementation

###### c++

```c++
class Solution {
private:
    vector<int> classes;
    vector<int> visit;
    vector<vector<int>> edges;

public:
    vector<int> findOrder(int numCourses, vector<vector<int>>& prerequisites) {
        classes.resize(numCourses);
        edges.resize(numCourses);
        visit.resize(numCourses);

        // constructing edges
        for (vector<int>& pre : prerequisites) {
            edges[pre[1]].push_back(pre[0]);
            visit[pre[0]]++;
        }

        // creating entry;
        vector<int> answer;
        for (int i = 0; i < numCourses; i++) {
            if (visit[i] == 0) answer.push_back(i);
        }

        // bfs
        int last_size = 0;
        int current_size = answer.size();
        while (last_size != current_size) {
            for (int i = last_size; i < current_size; i++) {
                const vector<int>& edge = edges[answer[i]]; 
                for (int cls : edge) { 
                    visit[cls]--;
                    if (visit[cls] == 0) answer.push_back(cls);
                }
            }

            last_size = current_size;
            current_size = answer.size();
        }

        if (answer.size() != numCourses) return {};
        return answer;
    }
};
```
