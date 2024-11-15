# 104. Maximum Depth of Binary Tree

### Description

Given the root of a binary tree, return its maximum depth.

A binary tree's maximum depth is the number of nodes along the longest path from the root node down to the farthest leaf node.

**Example: **

```
Input: root = [3,9,20,null,null,15,7]
Output: 3
```

```
Input: root = [1,null,2]
Output: 2
```

### Solution

深度优先搜索，遍历整个树

### Implementation

###### c++

```c++
class Solution {
public:
    int maxDepth(TreeNode* root) {
        if (root == nullptr) return 0;
        return dfs(root);
    }

private:
    int dfs(TreeNode* node) {
        if (node == nullptr) return 0;
        int leftDepth = dfs(node->left);
        int rightDepth = dfs(node->right);
        return 1 + max(leftDepth, rightDepth);
    }
};
```
