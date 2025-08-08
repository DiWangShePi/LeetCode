# 437. Path Sum III

### Description

Given the root of a binary tree and an integer targetSum, return the number of paths where the sum of the values along the path equals targetSum.

The path does not need to start or end at the root or a leaf, but it must go downwards (i.e., traveling only from parent nodes to child nodes).

### Example 

###### Example I

![](./pathsum3-1-tree.jpg)

> Input: root = [10,5,-3,3,2,null,11,3,-2,null,1], targetSum = 8
> Output: 3
> Explanation: The paths that sum to 8 are shown.

###### Example II

> Input: root = [5,4,8,11,null,13,4,7,2,null,null,5,1], targetSum = 22
> Output: 3

### Solution

我们先考虑暴力的方式：尝试以每一个节点作为起点，在其子树中寻找可能的解

```c++
class Solution {
public:
    int pathSum(TreeNode* root, int targetSum) {
        if (!root) return 0;
        return dfs(root, targetSum) 
               + pathSum(root->left, targetSum) 
               + pathSum(root->right, targetSum);
    }

private:
    int dfs(TreeNode* node, long long targetSum) {
        if (!node) return 0;
        int res = 0;
        if (node->val == targetSum) res++;

        res += dfs(node->left, targetSum - node->val);
        res += dfs(node->right, targetSum - node->val);
        return res;
    }
};
```

一个更优雅的解法是采用前缀和：遍历树，每到一个节点，记录当前节点到root节点的路径值之和，存储在一个字典中。
接下来，我们在字典中寻找是否存在targetSum - currentSum的值，存在即代表有一条这样的路径。

```c++
class Solution {
public:
    int pathSum(TreeNode* root, int targetSum) {
        unordered_map<long long, int> prefix;
        prefix[0] = 1; 
        return dfs(root, 0, targetSum, prefix);
    }

private:
    int dfs(TreeNode* node, long long currSum, int target, unordered_map<long long, int>& prefix) {
        if (!node) return 0;

        currSum += node->val;
        int res = prefix[currSum - target]; 

        prefix[currSum]++; 

        res += dfs(node->left, currSum, target, prefix);
        res += dfs(node->right, currSum, target, prefix);

        prefix[currSum]--; 
        return res;
    }
};
```
