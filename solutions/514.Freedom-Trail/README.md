# 514. Freedom Trail

### Description

In the video game Fallout 4, the quest "Road to Freedom" requires players to reach a metal dial called the "Freedom Trail Ring" and use the dial to spell a specific keyword to open the door.

Given a string ring that represents the code engraved on the outer ring and another string key that represents the keyword that needs to be spelled, return the minimum number of steps to spell all the characters in the keyword.

Initially, the first character of the ring is aligned at the "12:00" direction. You should spell all the characters in key one by one by rotating ring clockwise or anticlockwise to make each character of the string key aligned at the "12:00" direction and then by pressing the center button.

At the stage of rotating the ring to spell the key character key[i]:

1. You can rotate the ring clockwise or anticlockwise by one place, which counts as one step. The final purpose of the rotation is to align one of ring's characters at the "12:00" direction, where this character must equal key[i].
2. If the character key[i] has been aligned at the "12:00" direction, press the center button to spell, which also counts as one step. After the pressing, you could begin to spell the next character in the key (next stage). Otherwise, you have finished all the spelling.

### Example 

###### Example I

![](./ring.jpg)

> Input: ring = "godding", key = "gd"
> Output: 4
> Explanation:
> For the first key character 'g', since it is already in place, we just need 1 step to spell this character. 
> For the second key character 'd', we need to rotate the ring "godding" anticlockwise by two steps to make it become "ddinggo".
> Also, we need 1 more step for spelling.
> So the final output is 4.

###### Example II

> Input: ring = "godding", key = "godding"
> Output: 13

### Solution

可以用深度优先搜索加记忆化存储，在每一次枚举所有可能的选择，从中选择最优的哪一个。这里的最优并非是局部的，而是要考虑后面累计开销的最优解。

```c++
class Solution {
public:
    int findRotateSteps(string ring, string key) {
        int n = ring.size();
        unordered_map<char, vector<int>> dict;

        for (int i = 0; i < n; i++) 
            dict[ring[i]].push_back(i);

        vector<vector<int>> dp(n, vector<int>(key.size(), -1));
        return dfs(ring, key, 0, dict, 0, dp);
    }

private:
    int dfs(const string& ring, const string& key, int idx,
            const unordered_map<char, vector<int>>& dict,
            int pos, vector<vector<int>>& dp) {
        if (idx == key.size()) return 0;
        if (dp[pos][idx] != -1) return dp[pos][idx];

        int n = ring.size();
        int res = INT_MAX;

        for (int next : dict.at(key[idx])) {
            int diff = abs(pos - next);
            int step = min(diff, n - diff);
            res = min(res, step + 1 + dfs(ring, key, idx + 1, dict, next, dp));
        }

        dp[pos][idx] = res;
        return res;
    }
};
```

当然，我们也可以采用动态规划的方法，不同的地方在于：递归是先构建了尾部字符串的解，再回到上一层填入答案；而动态规划则是先针对key[0:i]给出一个最优解，再在此基础上尝试填入下一个字符的最优解。

```c++
int findRotateSteps(string ring, string key) {
    int n = ring.size(), m = key.size();
    unordered_map<char, vector<int>> pos;
    for (int i = 0; i < n; i++) pos[ring[i]].push_back(i);

    vector<vector<int>> dp(m, vector<int>(n, INT_MAX));
    for (int j : pos[key[0]]) {
        int diff = abs(j - 0);
        dp[0][j] = min(diff, n - diff) + 1;
    }

    for (int i = 1; i < m; i++) {
        for (int j : pos[key[i]]) {
            for (int k : pos[key[i-1]]) {
                int diff = abs(j - k);
                int step = min(diff, n - diff);
                dp[i][j] = min(dp[i][j], dp[i-1][k] + step + 1);
            }
        }
    }

    return *min_element(dp[m-1].begin(), dp[m-1].end());
}
```

由于动态规划中只依赖于上一层的状态，我们可以优化其空间存储

```c++
class Solution {
public:
    int findRotateSteps(string ring, string key) {
        int n = ring.size(), m = key.size();
        unordered_map<char, vector<int>> pos;

        for (int i = 0; i < n; i++) 
            pos[ring[i]].push_back(i);
        vector<int> prev(n, INT_MAX), curr(n, INT_MAX);

        for (int j : pos[key[0]]) {
            int diff = abs(j - 0);
            prev[j] = min(diff, n - diff) + 1;  
        }
        for (int i = 1; i < m; i++) {
            fill(curr.begin(), curr.end(), INT_MAX);
            for (int j : pos[key[i]]) { 
                for (int k : pos[key[i - 1]]) { 
                    // if (prev[k] == INT_MAX) continue;

                    int diff = abs(j - k);
                    int step = min(diff, n - diff);
                    curr[j] = min(curr[j], prev[k] + step + 1);
                }
            }
            prev.swap(curr);
        }

        return *min_element(prev.begin(), prev.end());
    }
};
```
