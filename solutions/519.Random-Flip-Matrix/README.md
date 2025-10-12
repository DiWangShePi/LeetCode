# 519. Random Flip Matrix

### Description

There is an m x n binary grid matrix with all the values set 0 initially. Design an algorithm to randomly pick an index (i, j) where matrix[i][j] == 0 and flips it to 1. All the indices (i, j) where matrix[i][j] == 0 should be equally likely to be returned.

Optimize your algorithm to minimize the number of calls made to the built-in random function of your language and optimize the time and space complexity.

Implement the Solution class:

- Solution(int m, int n) Initializes the object with the size of the binary matrix m and n.
- int[] flip() Returns a random index [i, j] of the matrix where matrix[i][j] == 0 and flips it to 1.
- void reset() Resets all the values of the matrix to be 0.

### Example 

###### Example I

> Input
> ["Solution", "flip", "flip", "flip", "reset", "flip"]
> [[3, 1], [], [], [], [], []]
> Output
> [null, [1, 0], [2, 0], [0, 0], null, [2, 0]]
> 
> Explanation
> Solution solution = new Solution(3, 1);
> solution.flip();  // return [1, 0], [0,0], [1,0], and [2,0] should be equally likely to be returned.
> solution.flip();  // return [2, 0], Since [1,0] was returned, [2,0] and [0,0]
> solution.flip();  // return [0, 0], Based on the previously returned indices, only [0,0] can be returned.
> solution.reset(); // All the values are reset to 0 and can be returned.
> solution.flip();  // return [2, 0], [0,0], [1,0], and [2,0] should be equally likely to be returned.

### Solution

这一题要求在每一次 reset 之前，出现过的值不能重复出现。因此我们可以用拒绝采样来实现。
即每次随机采样一个点，如果这个点出现过，就重新采样。

在第一版实验中，我们用一个数组来记录所有可能出现过的值。但当M和N比较大，采样次数比较小时，这样做会浪费很大的空间。

```c++
class Solution {
    vector<vector<int>> map;
    int M, N;

public:
    Solution(int m, int n) {
        M = m;
        N = n;
        map = vector<vector<int>>(M, vector<int>(N, 0));
    }
    
    vector<int> flip() {
        while (true) {
            int x = rand() % M;
            int y = rand() % N;
            if (map[x][y] == 0) {
                map[x][y] = 1;
                return {x, y};
            }
        }
    }
    
    void reset() {
        map = vector<vector<int>>(M, vector<int>(N, 0));
    }
};

/**
 * Your Solution object will be instantiated and called as such:
 * Solution* obj = new Solution(m, n);
 * vector<int> param_1 = obj->flip();
 * obj->reset();
 */
```

由于我们的目标仅仅是记录出现过的值，我们可以考虑用集合或者字典来实现。如果它不支持对vector的查重，我们也可以将值映射一下

> Implement it with set

```c++
class Solution {
    set<vector<int>> map;
    int M, N;

public:
    Solution(int m, int n) {
        M = m;
        N = n;
    }
    
    vector<int> flip() {
        while (true) {
            int x = rand() % M;
            int y = rand() % N;
            if (map.find({x, y}) == map.end()) {
                map.insert({x, y});
                return {x, y};
            }
        }
    }
    
    void reset() {
        map.clear();
    }
};

/**
 * Your Solution object will be instantiated and called as such:
 * Solution* obj = new Solution(m, n);
 * vector<int> param_1 = obj->flip();
 * obj->reset();
 */
```

> Implement it with unordered_map 

```c++
class Solution {
    unordered_map<int, bool> map;
    int M, N;

public:
    Solution(int m, int n) {
        M = m;
        N = n;
    }
    
    vector<int> flip() {
        while (true) {
            int x = rand() % M;
            int y = rand() % N;
            if (map.count(x * (N + 1) + y) == 0) {
                map[x * (N + 1) + y] = 1;
                return {x, y};
            }
        }
    }
    
    void reset() {
        map.clear();
    }
};

/**
 * Your Solution object will be instantiated and called as such:
 * Solution* obj = new Solution(m, n);
 * vector<int> param_1 = obj->flip();
 * obj->reset();
 */
```

拒绝采样在时间复杂度上可能不是最优的，假定我们在某一次采样中，数组里仅有一个可以被采样的值，我们会需要O(N)的时间复杂度来“找到”这个值。

为了解决这个问题，更快速的找到没有被采样的值，我们可以记录下来，未被采样的空间还有多大，直接在剩余的空间里采样。

```c++
class Solution {
public:
    Solution(int m, int n) {
        this->m = m;
        this->n = n;
        this->total = m * n;
        srand(time(nullptr));
    }
    
    vector<int> flip() {
        int x = rand() % total;
        vector<int> ans;
        total--;   

        // 查找位置 x 对应的映射
        if (map.count(x)) {
            ans = {map[x] / n, map[x] % n};
        } else {
            ans = {x / n, x % n};
        }
        
        // 将位置 x 对应的映射设置为位置 total 对应的映射
        if (map.count(total)) {
            map[x] = map[total];
        } else {
            map[x] = total;
        }
        return ans;
    }
    
    void reset() {
        total = m * n;
        map.clear();
    }
private:
    int m;
    int n;
    int total;
    unordered_map<int, int> map;
};
```
