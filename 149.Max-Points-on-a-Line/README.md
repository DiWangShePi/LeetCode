# 149. Max Points on a Line

### Description

Given an array of points where points[i] = [xi, yi] represents a point on the X-Y plane, return the maximum number of points that lie on the same straight line.

### Solution

每两个点取一次斜率，记录斜率的对应的数值，最终最多的加一即为最多的点的数目。

> 暴力 

### Implementation

###### c++

```c++
class Solution {
public:
    int maxPoints(vector<vector<int>>& points) {
        int n = points.size();
        if (n <= 2) return n;

        int an = 0;
        for (int i = 0; i < n; i++) {
            unordered_map<int, int> dict;
            for (int j = i + 1; j < n; j++) {
                int x = points[i][0] - points[j][0];
                int y = points[i][1] - points[j][1];

                if (x == 0) y = 1;
                else if (y == 0) x = 1;
                else {
                    if (y < 0) {x = -x, y = -y;}
                    int gcdXY = gcd(abs(x), abs(y));
                    x /= gcdXY, y /= gcdXY;
                }
                dict[y + x * 20001]++;
            }
            int maxn = 0;
            for(auto& [_, num] : dict) {
                maxn = max(maxn, num + 1);
            }
            an = max(an, maxn);
        }
        return an;
    }

private:
    int gcd(int a, int b) {
        return b ? gcd(b, a % b) : a;
    }
};
```
