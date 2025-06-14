# 386. Lexicographical Numbers

### Description

Given an integer n, return all the numbers in the range [1, n] sorted in lexicographical order.

You must write an algorithm that runs in O(n) time and uses O(1) extra space. 

### Example

###### Example I

```
Input: n = 13
Output: [1,10,11,12,13,2,3,4,5,6,7,8,9]
```

###### Example II

```
Input: n = 2
Output: [1,2]
```

### Solution

字典序的本质是 深度优先遍历（DFS）一棵十叉树，其中每个节点的子节点是该数字后面追加 0~9 形成的新数字。例如：

1 的子节点是 10, 11, 12, ..., 19;
10 的子节点是 100, 101, ..., 109;
2 的子节点是 20, 21, ..., 29


我们可以模拟这个 DFS 过程，按字典序生成数字：

从 1 开始，优先向下遍历（即 ×10，进入更深的位数，如 1 → 10 → 100）;
如果当前数字超过 n，就回退（比如 100 > 13，回退到 10），然后尝试 +1（10 → 11）;
如果 +1 后位数减少（比如 19 +1 = 20，但 2 比 19 少一位），就回退到更高层（19 → 2）。

```c++
class Solution {
public:
    vector<int> lexicalOrder(int n) {
        vector<int> res;
        int curr = 1;
        for (int i = 0; i < n; ++i) {
            res.push_back(curr);
            if (curr * 10 <= n) {
                curr *= 10;  
            } else {
                if (curr >= n) {
                    curr /= 10; 
                }
                curr += 1; 
                while (curr % 10 == 0) { 
                    curr /= 10;
                }
            }
        }
        return res;
    }
};
```
